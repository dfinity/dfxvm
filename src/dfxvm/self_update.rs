use crate::dist_manifest::lookup_latest_version;
use crate::download::{download_file, verify_checksum};
use crate::error::dfxvm::self_update::CleanupSelfUpdaterError;
use crate::error::dfxvm::{
    self_update::{
        DownloadLatestBinaryError,
        DownloadLatestBinaryError::CreateTempDirIn,
        ExtractBinaryError,
        ExtractBinaryError::{DfxvmNotFound, ReadArchiveEntries, UnpackBinary},
        FormatTarballUrlError, SelfReplaceError,
    },
    SelfUpdateError,
    SelfUpdateError::Exec,
};
use crate::fs::{open_file, remove_file};
use crate::installation::install_binaries;
use crate::locations::Locations;
use crate::settings::Settings;
use flate2::read::GzDecoder;
use reqwest::{Client, Url};
use std::os::unix::prelude::CommandExt;
use std::path::Path;
use tar::Archive;

pub async fn self_update(locations: &Locations) -> Result<(), SelfUpdateError> {
    info!("checking for self-update");
    let settings = Settings::load_or_default(&locations.settings_path())?;
    let latest_version = lookup_latest_version(&settings).await?;
    let our_version = env!("CARGO_PKG_VERSION");
    if latest_version == our_version {
        info!("dfxvm unchanged - {latest_version}");
        return Ok(());
    }

    info!("updating to {latest_version}");

    let tarball_url = format_tarball_url(&settings)?;
    let self_update_path = locations.self_update_path();

    download_latest_binary(&tarball_url, &self_update_path, locations).await?;

    let mut command = std::process::Command::new(self_update_path);
    command.arg("--self-replace");
    let err = command.exec();
    Err(Exec {
        command,
        source: err,
    })
}

pub fn self_replace(locations: &Locations) -> Result<(), SelfReplaceError> {
    install_binaries(&locations.bin_dir())?;
    Ok(())
}

// called on next execution of dfx or dfxvm
pub fn cleanup_self_updater(locations: &Locations) -> Result<(), CleanupSelfUpdaterError> {
    let path = locations.self_update_path();

    if path.exists() {
        remove_file(&path)?;
    }

    Ok(())
}

fn format_tarball_url(settings: &Settings) -> Result<Url, FormatTarballUrlError> {
    #[cfg(target_arch = "aarch64")]
    let architecture = "aarch64-apple-darwin";
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    let architecture = "x86_64-apple-darwin";
    #[cfg(target_os = "linux")]
    let architecture = "x86_64-unknown-linux-gnu";

    let basename = format!("dfxvm-{}", architecture);
    let url = format!(
        "{}/{basename}.tar.gz",
        settings.dfxvm_latest_download_root()
    );

    Url::parse(&url).map_err(|source| FormatTarballUrlError { url, source })
}

async fn download_latest_binary(
    tarball_url: &Url,
    binary_path: &Path,
    locations: &Locations,
) -> Result<(), DownloadLatestBinaryError> {
    let shasum_url = Url::parse(&format!("{tarball_url}.sha256"))?;

    let download_dir = tempfile::Builder::new()
        .prefix("dfxvm-download")
        .tempdir_in(locations.data_local_dir())
        .map_err(|source| CreateTempDirIn {
            path: locations.data_local_dir().to_path_buf(),
            source,
        })?;

    let downloaded_tarball_path = download_dir.path().join("dfxvm.tar.gz");
    let downloaded_shasum_path = download_dir.path().join("dfxvm.tar.gz.sha256");

    let client = Client::new();

    download_file(&client, &shasum_url, &downloaded_shasum_path).await?;
    let computed_hash = download_file(&client, tarball_url, &downloaded_tarball_path).await?;
    verify_checksum(computed_hash, &downloaded_shasum_path)?;

    extract_binary(binary_path, &downloaded_tarball_path)?;
    Ok(())
}

fn extract_binary(
    binary_path: &Path,
    downloaded_tarball_path: &Path,
) -> Result<(), ExtractBinaryError> {
    let tar_gz = open_file(downloaded_tarball_path)?;
    let tar = GzDecoder::new(tar_gz);

    Archive::new(tar)
        .entries()
        .map_err(ReadArchiveEntries)?
        .enumerate()
        .filter_map(|(_i, entry)| entry.ok())
        .find(|entry| {
            entry
                .header()
                .path()
                .ok()
                .as_ref()
                .and_then(|x| x.to_str())
                .map(|str_path| str_path.ends_with("dfxvm"))
                .unwrap_or(false)
        })
        .ok_or(DfxvmNotFound)?
        .unpack(binary_path)
        .map_err(UnpackBinary)?;
    Ok(())
}
