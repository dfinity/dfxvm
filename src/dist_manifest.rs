use crate::error::dfxvm::self_update::LookupLatestVersionError;
use crate::json::fetch_json;
use crate::settings::Settings;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug)]
struct Release {
    app_name: String,
    app_version: String,
}

#[derive(Deserialize, Debug)]
struct DistManifest {
    releases: Vec<Release>,
}

pub async fn lookup_latest_version(
    settings: &Settings,
) -> Result<String, LookupLatestVersionError> {
    let dist_manifest_url = format!(
        "{}/dist-manifest.json",
        settings.dfxvm_latest_download_root()
    );
    let url =
        Url::parse(&dist_manifest_url).map_err(|source| LookupLatestVersionError::ParseUrl {
            url: dist_manifest_url,
            source,
        })?;
    let dist_manifest = fetch_json::<DistManifest>(&url).await?;
    let dfxvm_release = dist_manifest
        .releases
        .iter()
        .find(|release| release.app_name == "dfxvm")
        .ok_or(LookupLatestVersionError::NoDfxvmRelease { url })?;
    let latest_version = dfxvm_release.app_version.clone();
    Ok(latest_version)
}
