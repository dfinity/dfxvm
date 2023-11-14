use crate::error::{
    dfxvm::install::{
        DownloadFileError,
        DownloadFileError::{DownloadContents, GetContentLength},
        DownloadVerifiedTarballError,
        DownloadVerifiedTarballError::{DownloadFile, NoSuchVersion},
        ExtractArchiveError,
        ExtractArchiveError::Unpack,
        InstallError,
        InstallError::{CreateTempDir, CreateTempDirIn},
        VerifyChecksumError,
        VerifyChecksumError::{HashMismatch, MalformedChecksumFile},
        WrappedReqwestError,
    },
    fs::WriteFileError,
    Retryable,
};

use crate::fs::{create_dir_all, create_file, open_file, read_to_string, rename};
use crate::locations::Locations;
use crate::log::log_error;
use crate::settings::Settings;
use backoff::{future::retry_notify, ExponentialBackoff};
use flate2::read::GzDecoder;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, StatusCode, Url};
use semver::Version;
use sha2::{Digest, Sha256};
use std::cmp::min;
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::Archive;
use tempdir::TempDir;

pub fn installed(version: &Version, locations: &Locations) -> bool {
    locations.version_dir(version).exists()
}

pub async fn install(version: Version) -> Result<(), InstallError> {
    let locations = Locations::new()?;
    let settings = Settings::load_or_default(&locations.settings_path())?;
    let version_dir = locations.version_dir(&version);
    if installed(&version, &locations) {
        info!("dfx {version} is already installed");
        return Ok(());
    }
    create_dir_all(locations.versions_dir())?;

    info!("installing dfx {version}");

    let download_dir = TempDir::new("dfxvm-download").map_err(CreateTempDir)?;

    let downloaded_tarball_path =
        download_verified_tarball(&version, download_dir.path(), &settings).await?;

    let install_dir = TempDir::new_in(locations.versions_dir(), ".install").map_err(|source| {
        CreateTempDirIn {
            path: locations.versions_dir().to_path_buf(),
            source,
        }
    })?;

    extract_binary(&downloaded_tarball_path, install_dir.path())?;

    rename(install_dir.path(), &version_dir)?;

    info!("installed dfx {version}");

    Ok(())
}

async fn download_verified_tarball(
    version: &Version,
    download_dir: &Path,
    settings: &Settings,
) -> Result<PathBuf, DownloadVerifiedTarballError> {
    let tarball_filename = "dfx.tar.gz";
    let shasum_filename = format!("{tarball_filename}.sha256");
    let downloaded_tarball_path = download_dir.join(tarball_filename);
    let downloaded_shasum_path = download_dir.join(shasum_filename);

    let tarball_url = format_tarball_url(version, settings)?;
    let shasum_url = Url::parse(&format!("{tarball_url}.sha256"))?;

    let client = Client::new();

    // download the shasum file first because it's smaller
    download_file(&client, &shasum_url, &downloaded_shasum_path)
        .await
        .map_err(|e| match e {
            DownloadFileError::Status(WrappedReqwestError(status_err))
                if matches!(status_err.status(),
                    Some(status) if status == StatusCode::NOT_FOUND) =>
            {
                NoSuchVersion(WrappedReqwestError(status_err))
            }
            other => DownloadFile(other),
        })?;

    let computed_hash = download_file(&client, &tarball_url, &downloaded_tarball_path).await?;

    verify_checksum(computed_hash, &downloaded_shasum_path)?;

    Ok(downloaded_tarball_path)
}

fn format_tarball_url(version: &Version, settings: &Settings) -> Result<Url, url::ParseError> {
    #[cfg(target_os = "linux")]
    let platform = "linux";
    #[cfg(target_os = "macos")]
    let platform = "darwin";
    let arch = "x86_64";

    let url_template = settings.download_url_template();
    let url = url_template
        .replace("{{version}}", &version.to_string())
        .replace("{{arch}}", arch)
        .replace("{{platform}}", platform);
    Url::parse(&url)
}

fn extract_binary(tar_gz_path: &Path, dest: &Path) -> Result<(), ExtractArchiveError> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{msg} {spinner}"));
    spinner.set_message("extracting archive...");
    spinner.enable_steady_tick(100);

    let tar_gz = open_file(tar_gz_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(dest).map_err(|source| Unpack {
        path: dest.to_path_buf(),
        source,
    })?;

    spinner.finish_and_clear();
    info!("extracted archive");
    Ok(())
}

struct FileHash(String);

fn verify_checksum(hash: FileHash, shasum_path: &Path) -> Result<(), VerifyChecksumError> {
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

async fn download_file(
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
