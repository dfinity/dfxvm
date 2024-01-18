use crate::error::{
    json::{
        FetchJsonDocError,
        FetchJsonDocError::{Get, Parse, ReadBytes, Status},
        LoadJsonFileError, SaveJsonFileError,
    },
    reqwest::WrappedReqwestError,
    Retryable,
};
use crate::fs::read;
use crate::log::log_error;
use backoff::{future::retry_notify, ExponentialBackoff};
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::Path;

pub fn load_json_file<T: for<'a> serde::de::Deserialize<'a>>(
    path: &Path,
) -> Result<T, LoadJsonFileError> {
    let content = read(path)?;

    serde_json::from_slice(content.as_ref()).map_err(|source| LoadJsonFileError::Parse {
        path: path.to_path_buf(),
        source,
    })
}

pub fn save_json_file<T: Serialize>(path: &Path, value: &T) -> Result<(), SaveJsonFileError> {
    let content =
        serde_json::to_string_pretty(&value).map_err(|source| SaveJsonFileError::Serialize {
            path: path.to_path_buf(),
            source,
        })?;
    crate::fs::write(path, content)?;
    Ok(())
}

pub async fn fetch_json<T: DeserializeOwned>(url: &Url) -> Result<T, FetchJsonDocError> {
    let client = Client::new();
    let notify = |err, dur| {
        log_error(&err);
        err!("retry in {dur:?}");
    };

    let operation = || async {
        match attempt_fetch_json(&client, url.clone()).await {
            Ok(doc) => Ok(doc),
            Err(e) if e.is_retryable() => Err(backoff::Error::transient(e)),
            Err(e) => Err(backoff::Error::permanent(e)),
        }
    };

    let backoff = ExponentialBackoff::default();
    retry_notify(backoff, operation, notify).await
}

async fn attempt_fetch_json<T: DeserializeOwned>(
    client: &Client,
    url: Url,
) -> Result<T, FetchJsonDocError> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| Get(WrappedReqwestError(e)))?
        .error_for_status()
        .map_err(|e| Status(WrappedReqwestError(e)))?;
    let bytes = response
        .bytes()
        .await
        .map_err(|e| ReadBytes(WrappedReqwestError(e)))?;
    let doc = serde_json::from_slice(&bytes).map_err(Parse)?;
    Ok(doc)
}
