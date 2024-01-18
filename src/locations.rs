use crate::env::home_dir;
use crate::error::env::NoHomeDirectoryError;
use directories::ProjectDirs;
use semver::Version;
use std::path::{Path, PathBuf};

const SETTINGS_FILENAME: &str = "version-manager.json";

pub struct Locations {
    data_local_dir: PathBuf,
    versions_dir: PathBuf,
    config_dir: PathBuf,
}

impl Locations {
    pub fn versions_dir(&self) -> &Path {
        &self.versions_dir
    }

    pub fn version_dir(&self, version: &Version) -> PathBuf {
        self.versions_dir().join(version.to_string())
    }

    pub fn dfx_bin_path(&self, version: &Version) -> PathBuf {
        self.version_dir(version).join("dfx")
    }

    pub fn self_update_path(&self) -> PathBuf {
        self.bin_dir()
            .join("dfxvm-init-self-update")
    }

    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }
    pub fn data_local_dir(&self) -> &Path {
        &self.data_local_dir
    }
    pub fn bin_dir(&self) -> PathBuf {
        self.data_local_dir.join("bin")
    }

    pub fn settings_path(&self) -> PathBuf {
        self.config_dir.join(SETTINGS_FILENAME)
    }
}

impl Locations {
    pub fn new() -> Result<Self, NoHomeDirectoryError> {
        let project_dirs =
            ProjectDirs::from("org", "dfinity", "dfx").ok_or(NoHomeDirectoryError)?;
        let data_local_dir = project_dirs.data_local_dir().to_path_buf();
        let versions_dir = data_local_dir.join("versions");
        #[cfg(unix)]
            let config_dir = home_dir()?.join(".config").join("dfx");
        Ok(Self {
            data_local_dir,
            versions_dir,
            config_dir,
        })
    }
}
