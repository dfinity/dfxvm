use crate::dfxvm::install::{install, installed};
use crate::error::dfxvm::default::{
    DefaultError, DisplayDefaultError, DisplayDefaultError::NoDefaultVersion, SetDefaultError,
};
use crate::fs::create_dir_all;
use crate::locations::Locations;
use crate::settings::Settings;
use semver::Version;

pub async fn default(version: Option<Version>, locations: &Locations) -> Result<(), DefaultError> {
    if let Some(version) = version {
        set_default(&version, locations).await?;
    } else {
        display_default(locations)?;
    }
    Ok(())
}

pub async fn set_default(version: &Version, locations: &Locations) -> Result<(), SetDefaultError> {
    if installed(version, locations) {
        info!("using existing install for dfx {version}");
    } else {
        install(version.clone(), locations).await?;
    }

    let path = locations.settings_path();
    let mut settings = Settings::load_or_default(&path)?;

    if matches!(settings.default_version, Some(ref v) if v == version) {
        info!("dfx {} is already the default version", version);
    } else {
        settings.default_version = Some(version.clone());

        create_dir_all(locations.config_dir())?;
        settings.save(&path)?;

        info!("set default version to dfx {}", version);
    }
    Ok(())
}

pub fn display_default(locations: &Locations) -> Result<(), DisplayDefaultError> {
    let path = locations.settings_path();
    let settings = Settings::load_or_default(&path)?;

    let default_version = settings.default_version.ok_or(NoDefaultVersion)?;
    println!("{}", default_version);
    Ok(())
}
