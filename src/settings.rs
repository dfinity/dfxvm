use crate::error::json::LoadJsonFileError;
use crate::json::load_json_file;
use semver::Version;
use serde::Deserialize;
use std::path::Path;

const DEFAULT_DOWNLOAD_URL_TEMPLATE: &str = "https://github.com/dfinity/sdk/releases/download/{{version}}/dfx-{{version}}-{{arch}}-{{platform}}.tar.gz";

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Settings {
    pub default_version: Option<Version>,

    download_url_template: Option<String>,
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
}
