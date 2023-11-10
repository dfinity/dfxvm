use crate::error::json::LoadJsonFileError;
use crate::json::load_json_file;
use semver::Version;
use serde::Deserialize;
use std::path::Path;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Settings {
    pub default_version: Option<Version>,
}

impl Settings {
    pub fn load_or_default(path: &Path) -> Result<Self, LoadJsonFileError> {
        if path.exists() {
            load_json_file(path)
        } else {
            Ok(Self::default())
        }
    }
}
