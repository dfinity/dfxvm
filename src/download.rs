use crate::error::{
    download::{
        DownloadFileError,
        DownloadFileError::{DownloadContents, GetContentLength},
        VerifyChecksumError,
        VerifyChecksumError::{HashMismatch, MalformedChecksumFile},
    },
    fs::WriteFileError,
    reqwest::WrappedReqwestError,
    Retryable,
};
use crate::fs::{create_file, read_to_string};
use crate::log::log_error;
use backoff::future::retry_notify;
use backoff::ExponentialBackoff;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::cmp::min;
use std::io::Write;
use std::path::Path;
use url::Url;

pub struct FileHash(String);

pub fn verify_checksum(hash: FileHash, shasum_path: &Path) -> Result<(), VerifyChecksumError> {
    let contents = read_to_string(shasum_path)?;
    let contents_to_split = contents.clone();
    let mut parts = contents_to_split.split_whitespace();
    let expected = parts
        .next()
        .ok_or(MalformedChecksumFile { contents })?
        .to_string();
    let actual = hash.0;
    if expected != actual {
        return Err(HashMismatch { expected, actual });
    }

    info!("verified checksum {}", actual);
    Ok(())
}

pub async fn download_file(
    client: &Client,
    url: &Url,
    path: &Path,
) -> Result<FileHash, DownloadFileError> {
    let notify = |err, dur| {
        log_error(&err);
        err!("retry in {dur:?}");
    };

    let operation = || async {
        match attempt_download_file(client, url, path).await {
            Ok(file_hash) => Ok(file_hash),
            Err(e) if e.is_retryable() => Err(backoff::Error::transient(e)),
            Err(e) => Err(backoff::Error::permanent(e)),
        }
    };

    let backoff = ExponentialBackoff::default();
    retry_notify(backoff, operation, notify).await
}

// h/t https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
async fn attempt_download_file(
    client: &Client,
    url: &Url,
    path: &Path,
) -> Result<FileHash, DownloadFileError> {
    info!("downloading {}", url);

    let pb = ProgressBar::new(0);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));

    let res = client
        .get(url.clone())
        .send()
        .await
        .map_err(|e| DownloadFileError::Get(WrappedReqwestError(e)))?
        .error_for_status()
        .map_err(|e| DownloadFileError::Status(WrappedReqwestError(e)))?;
    let total_size = res.content_length().ok_or(GetContentLength {
        url: url.to_string(),
    })?;
    pb.set_length(total_size);

    // download chunks
    let mut file = create_file(path)?;
    let mut stream = res.bytes_stream();

    let mut downloaded: u64 = 0;
    let mut sha256: Sha256 = Sha256::new();

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|source| DownloadContents {
            url: url.to_string(),
            source: WrappedReqwestError(source),
        })?;
        sha256.update(&chunk);
        file.write_all(&chunk).map_err(|source| WriteFileError {
            path: path.to_path_buf(),
            source,
        })?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish();
    info!("downloaded {}", url);
    let hash = hex::encode(sha256.finalize());
    Ok(FileHash(hash))
}
