use crate::error::json::{LoadJsonFileError, SaveJsonFileError};
use crate::json::{load_json_file, save_json_file};
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

const DEFAULT_DOWNLOAD_URL_TEMPLATE: &str = "https://github.com/dfinity/sdk/releases/download/{{version}}/dfx-{{version}}-{{arch}}-{{platform}}.tar.gz";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<Version>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url_template: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

impl Settings {
    pub fn download_url_template(&self) -> String {
        self.download_url_template
            .clone()
            .unwrap_or_else(|| DEFAULT_DOWNLOAD_URL_TEMPLATE.to_string())
    }

    pub fn load_or_default(path: &Path) -> Result<Self, LoadJsonFileError> {
        if path.exists() {
            load_json_file(path)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), SaveJsonFileError> {
        save_json_file(path, &self)
    }
}
