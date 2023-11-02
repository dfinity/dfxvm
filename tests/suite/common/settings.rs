use serde_json::json;
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
        let json = json!({
            "default_version": version
        });
        let s = serde_json::to_string_pretty(&json).unwrap();
        self.write(&s);
    }

    pub fn write(&self, s: &str) {
        create_dir_all(self.path.parent().unwrap()).unwrap();
        std::fs::write(&self.path, s).unwrap();
    }
}
