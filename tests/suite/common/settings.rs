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

    pub fn write_default_version(&self, version: &str) {
        self.set_field("default_version", version);
    }

    pub fn write_download_url_template(&self, url_template: &str) {
        self.set_field("download_url_template", url_template);
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
