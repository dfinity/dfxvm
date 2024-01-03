use std::os::unix::prelude::CommandExt;
use flate2::read::GzDecoder;
use serde::Deserialize;
use crate::error::dfxvm::SelfUpdateError;
use crate::json::fetch_json_doc;
use crate::locations::Locations;
use crate::settings::Settings;
use reqwest::{Client, Url};
use semver::Version;
use tar::Archive;
use crate::download::{download_file, verify_checksum};
use crate::error::dfx::Error::Exec;
use crate::error::dfxvm::InstallError::CreateTempDir;
use crate::error::dfxvm::SelfUpdateError::CreateTempDirIn;
use crate::fs::open_file;
use crate::installation::install_binaries;

pub async fn self_update() -> Result<(), SelfUpdateError> {
    println!("update dfxvm to latest");
    let locations = Locations::new().unwrap();
    let settings = Settings::load_or_default(&locations.settings_path()).unwrap();
    let latest_version = get_latest_version(&settings).await.unwrap();
    println!("latest_version: {latest_version}");
    let our_version = env!("CARGO_PKG_VERSION");
    println!("our_version: {:?}", our_version);
    if latest_version == our_version {
        info!("already up to date");
        return Ok(());
    }

    info!("updating to {latest_version}");
    // download tarball
    let tarball_url = format_tarball_url(&settings);
    info!("url is {tarball_url}");
    let shasum_url = Url::parse(&format!("{tarball_url}.sha256")).unwrap();
    info!("shasum_url is {shasum_url}");

    let download_dir = tempfile::Builder::new()
        .prefix("dfxvm-download")
        .tempdir_in(locations.data_local_dir())
        .map_err(|source|CreateTempDirIn { path: locations.data_local_dir().to_path_buf(), source })?;

    let downloaded_tarball_path = download_dir.path().join("dfxvm.tar.gz");
    let downloaded_shasum_path = download_dir.path().join("dfxvm.tar.gz.sha256");

    let client = Client::new();

    download_file(&client, &shasum_url, &downloaded_shasum_path).await.unwrap();
    let computed_hash = download_file(&client, &tarball_url, &downloaded_tarball_path).await.unwrap();
    verify_checksum(computed_hash, &downloaded_shasum_path).unwrap();

    let tar_gz = open_file(&downloaded_tarball_path).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut ar = Archive::new(tar);

    let dfxvm_init_path = locations.data_local_dir().join("bin").join("dfxvm-init-2");
    for (i, file) in ar.entries().unwrap().enumerate() {
        let mut file = file.unwrap();
        info!("file-{}: {:?}", i, file.header().path().unwrap());
        if file.header().path().unwrap().to_str().unwrap().ends_with("dfxvm") {
            info!("found dfxvm, copying to {}", dfxvm_init_path.display());
            file.unpack(&dfxvm_init_path).unwrap();
            break;
        }
    }

    download_dir.close().unwrap();

    info!("calling self-replace");
    let mut command = std::process::Command::new(dfxvm_init_path);
    command.arg("--self-replace");
    let _err = command.exec();
/*    Err(Exec {
        command,
        source: err,
    })
*/


    Ok(())
}

pub fn self_replace() {
    info!("in self_replace");
    let locations = Locations::new().unwrap();

    install_binaries(&locations.data_local_dir().join("bin")).unwrap();
}

#[derive(Deserialize, Debug)]
struct Release {
    app_name: String,
    app_version: String,
}

#[derive(Deserialize, Debug)]
struct DistManifest {
    releases: Vec<Release>,
}

async fn get_latest_version(settings: &Settings) -> Result<String, SelfUpdateError> {
    let dist_manifest_url = format!("{}/dist-manifest.json", settings.dfxvm_latest_download_root());
    let url = Url::parse(&dist_manifest_url).unwrap();
    let dist_manifest = fetch_json_doc::<DistManifest>(&url).await.unwrap();
    println!("dist_manifest: {:?}", dist_manifest);
    let dfxvm_release = dist_manifest.releases.iter().find(|release| release.app_name == "dfxvm").unwrap();
    println!("dfxvm_release: {:?}", dfxvm_release);
    let latest_version = dfxvm_release.app_version.clone();
    println!("latest_version: {:?}", latest_version);
    Ok(latest_version)
}

fn format_tarball_url(settings: &Settings) -> Url {
    #[cfg(target_arch = "aarch64")]
        let arch = "aarch64-apple-darwin";
    #[cfg(all(target_os="macos", target_arch = "x86_64"))]
        let arch = "x86_64-apple-darwin";
    #[cfg(target_os = "linux")]
        let arch = "x86_64-unknown-linux-gnu";

    let basename = format!("dfxvm-{}", arch);
    let url = format!("{}/{basename}.tar.gz", settings.dfxvm_latest_download_root());

    Url::parse(&url).unwrap()
}