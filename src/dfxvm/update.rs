use crate::dfxvm::default::set_default;
use crate::error::{
    dfxvm::update::{
        FetchManifestError,
        FetchManifestError::{Get, Parse, ReadBytes, Status},
        UpdateError,
    },
    reqwest::WrappedReqwestError,
    Retryable,
};
use crate::locations::Locations;
use crate::log::log_error;
use crate::settings::Settings;
use backoff::{future::retry_notify, ExponentialBackoff};
use reqwest::{Client, Url};
use semver::Version;
use serde::Deserialize;

pub async fn update() -> Result<(), UpdateError> {
    let locations = Locations::new()?;
    let settings = Settings::load_or_default(&locations.settings_path())?;
    let url = Url::parse(&settings.manifest_url())?;

    info!("fetching {url}");
    let manifest = fetch_manifest(&url).await?;

    let latest_version = manifest.tags.latest;
    info!("latest dfx version is {latest_version}");

    set_default(latest_version, &locations).await?;

    Ok(())
}

#[derive(Deserialize)]
struct Tags {
    latest: Version,
}

#[derive(Deserialize)]
struct Manifest {
    tags: Tags,
}

async fn fetch_manifest(url: &Url) -> Result<Manifest, FetchManifestError> {
    let client = Client::new();
    let notify = |err, dur| {
        log_error(&err);
        err!("retry in {dur:?}");
    };

    let operation = || async {
        match attempt_fetch_manifest(&client, url.clone()).await {
            Ok(manifest) => Ok(manifest),
            Err(e) if e.is_retryable() => Err(backoff::Error::transient(e)),
            Err(e) => Err(backoff::Error::permanent(e)),
        }
    };

    let backoff = ExponentialBackoff::default();
    retry_notify(backoff, operation, notify).await
}

async fn attempt_fetch_manifest(client: &Client, url: Url) -> Result<Manifest, FetchManifestError> {
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
    let manifest = serde_json::from_slice(&bytes).map_err(Parse)?;
    Ok(manifest)
}
