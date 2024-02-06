use crate::error::json::{LoadJsonFileError, SaveJsonFileError};
use crate::json::{load_json_file, save_json_file};
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

const DEFAULT_DOWNLOAD_URL_TEMPLATE: &str =
    "https://github.com/dfinity/sdk/releases/download/{{version}}/{{basename}}.{{archive-format}}";
const DEFAULT_DFXVM_LATEST_DOWNLOAD_ROOT_URL: &str =
    "https://github.com/dfinity/dfxvm/releases/latest/download";
const DEFAULT_MANIFEST_URL: &str = "https://sdk.dfinity.org/manifest.json";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<Version>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dfxvm_latest_download_root: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    download_url_template: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    manifest_url: Option<String>,

    #[serde(flatten)]
    extra: Value,
}

impl Settings {
    pub fn dfxvm_latest_download_root(&self) -> String {
        self.dfxvm_latest_download_root
            .clone()
            .unwrap_or_else(|| DEFAULT_DFXVM_LATEST_DOWNLOAD_ROOT_URL.to_string())
    }

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

    pub fn manifest_url(&self) -> String {
        self.manifest_url
            .clone()
            .unwrap_or_else(|| DEFAULT_MANIFEST_URL.to_string())
    }

    pub fn save(&self, path: &Path) -> Result<(), SaveJsonFileError> {
        save_json_file(path, &self)
    }
}
