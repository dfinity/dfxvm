use crate::error::json::LoadJsonFileError;
use crate::json::load_json_file;
use semver::Version;
use serde::Deserialize;
use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub default_version: Option<Version>,
}

impl Settings {
    pub fn load(path: &Path) -> Result<Self, LoadJsonFileError> {
        load_json_file(path)
    }
}
