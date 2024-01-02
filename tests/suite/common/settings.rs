use itertools::Itertools;
use serde_json::{json, Value};
use std::fs::create_dir_all;
use std::path::PathBuf;

pub struct Settings {
    path: PathBuf,
}

impl Settings {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn read_default_version(&self) -> String {
        self.read()["default_version"].as_str().unwrap().to_string()
    }

    pub(crate) fn sorted_keys(&self) -> Vec<String> {
        self.read()
            .as_object()
            .unwrap()
            .keys()
            .sorted()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn write_default_version(&self, version: &str) {
        self.set_field("default_version", version);
    }

    pub fn write_download_url_template(&self, url_template: &str) {
        self.set_field("download_url_template", url_template);
    }

    pub fn write_manifest_url(&self, url_template: &str) {
        self.set_field("manifest_url", url_template);
    }

    pub fn write_dfxvm_latest_download_root_url(&self, url_template: &str) {
        self.set_field("dfxvm_latest_download_root", url_template);
    }

    pub fn write(&self, s: &str) {
        create_dir_all(self.path.parent().unwrap()).unwrap();
        std::fs::write(&self.path, s).unwrap();
    }

    fn set_field(&self, name: &str, value: &str) {
        let json = self.read();
        let mut json = json.as_object().unwrap().clone();
        json.insert(name.to_string(), json!(value));

        let formatted = serde_json::to_string_pretty(&json).unwrap();
        self.write(&formatted);
    }

    fn read(&self) -> Value {
        if self.path.exists() {
            let content = std::fs::read_to_string(&self.path).unwrap();
            serde_json::from_str(&content).unwrap()
        } else {
            json!({})
        }
    }
}
