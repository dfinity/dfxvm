use crate::error::settings::LoadSettingsError;
use crate::error::settings::LoadSettingsError::ParseDefaultVersion;
use crate::json::load_json_file;
use semver::Version;
use serde::Deserialize;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Settings {
    pub default_version: Version,
}

#[derive(Clone, Debug, Deserialize)]
struct SettingsJson {
    default_version: String,
}

impl Settings {
    pub fn load(path: &Path) -> Result<Self, LoadSettingsError> {
        let json: SettingsJson = load_json_file(path)?;
        let default_version =
            Version::parse(&json.default_version).map_err(|source| ParseDefaultVersion {
                version: json.default_version,
                path: path.to_path_buf(),
                source,
            })?;
        Ok(Self { default_version })
    }
}
