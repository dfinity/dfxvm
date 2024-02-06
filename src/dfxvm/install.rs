use crate::download::{download_file, verify_checksum};
use crate::error::{
    dfxvm::install::{
        ExtractArchiveError,
        ExtractArchiveError::Unpack,
        InstallError,
        InstallError::{CreateTempDir, CreateTempDirIn},
    },
    download::{
        DownloadFileError, DownloadVerifiedTarballError,
        DownloadVerifiedTarballError::{DownloadFile, NoSuchVersion},
    },
    reqwest::WrappedReqwestError,
};
use crate::fs::{create_dir_all, open_file, rename};
use crate::locations::Locations;
use crate::settings::Settings;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, StatusCode, Url};
use semver::Version;
use std::path::{Path, PathBuf};
use tar::Archive;

pub fn installed(version: &Version, locations: &Locations) -> bool {
    locations.version_dir(version).exists()
}

pub async fn install(version: Version, locations: &Locations) -> Result<(), InstallError> {
    let settings = Settings::load_or_default(&locations.settings_path())?;
    let version_dir = locations.version_dir(&version);
    if installed(&version, locations) {
        info!("dfx {version} is already installed");
        return Ok(());
    }
    create_dir_all(locations.versions_dir())?;

    info!("installing dfx {version}");

    let download_dir = tempfile::Builder::new()
        .prefix("dfxvm-download")
        .tempdir()
        .map_err(CreateTempDir)?;

    let downloaded_tarball_path =
        download_verified_tarball(&version, download_dir.path(), &settings).await?;

    let install_dir = tempfile::Builder::new()
        .prefix(".install")
        .tempdir_in(locations.versions_dir())
        .map_err(|source| CreateTempDirIn {
            path: locations.versions_dir().to_path_buf(),
            source,
        })?;

    extract_binary(&downloaded_tarball_path, install_dir.path())?;

    let tarball_basename = format_tarball_basename();
    rename(&install_dir.path().join(tarball_basename), &version_dir)?;

    info!("installed dfx {version}");

    Ok(())
}

async fn download_verified_tarball(
    version: &Version,
    download_dir: &Path,
    settings: &Settings,
) -> Result<PathBuf, DownloadVerifiedTarballError> {
    let tarball_basename = format_tarball_basename();
    let tarball_filename = format!("{tarball_basename}.tar.gz");
    let shasum_filename = format!("{tarball_filename}.sha256");
    let downloaded_tarball_path = download_dir.join(tarball_filename);
    let downloaded_shasum_path = download_dir.join(shasum_filename);

    let tarball_url = format_tarball_url(version, tarball_basename, settings)?;
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

fn format_tarball_basename() -> &'static str {
    #[cfg(target_os = "linux")]
    let basename = "dfx-x86_64-unknown-linux-gnu";
    #[cfg(target_os = "macos")]
    let basename = "dfx-x86_64-apple-darwin";
    basename
}

fn format_tarball_url(
    version: &Version,
    basename: &str,
    settings: &Settings,
) -> Result<Url, url::ParseError> {
    let url_template = settings.download_url_template();
    let url = url_template
        .replace("{{version}}", &version.to_string())
        .replace("{{basename}}", basename)
        .replace("{{archive-format}}", "tar.gz");
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
