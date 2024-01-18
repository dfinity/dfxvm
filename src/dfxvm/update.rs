use crate::dfxvm::default::set_default;
use crate::error::dfxvm::UpdateError;
use crate::json::fetch_json;
use crate::locations::Locations;
use crate::settings::Settings;
use reqwest::Url;
use semver::Version;
use serde::Deserialize;

pub async fn update(locations: &Locations) -> Result<(), UpdateError> {
    let settings = Settings::load_or_default(&locations.settings_path())?;
    let url = Url::parse(&settings.manifest_url())?;

    info!("fetching {url}");
    let manifest = fetch_json::<Manifest>(&url).await?;

    let latest_version = manifest.tags.latest;
    info!("latest dfx version is {latest_version}");

    set_default(&latest_version, locations).await?;

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
