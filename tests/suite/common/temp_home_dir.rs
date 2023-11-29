use crate::common::executable::{create_executable, wait_until_file_is_not_busy};
use crate::common::{dfxvm_path, file_contents, project_dirs, Settings};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

pub struct TempHomeDir {
    tempdir: TempDir,
    xdg_data_home: Option<PathBuf>,
}

impl TempHomeDir {
    pub fn new() -> Self {
        let tempdir = tempfile::Builder::new()
            .prefix("dfxvm-integration-tests-home")
            .tempdir()
            .unwrap();
        let xdg_data_home = None;
        Self {
            tempdir,
            xdg_data_home,
        }
    }

    pub fn with_xdg_data_home(self, xdg_data_home: &Path) -> Self {
        Self {
            xdg_data_home: Some(xdg_data_home.to_path_buf()),
            ..self
        }
    }

    pub fn path(&self) -> &Path {
        self.tempdir.path()
    }

    pub fn dfx(&self) -> Command {
        self.dfxvm_as_command_named("dfx")
    }

    pub fn dfxvm(&self) -> Command {
        self.dfxvm_as_command_named("dfxvm")
    }

    pub fn dfxvm_init(&self) -> Command {
        self.dfxvm_as_command_named("dfxvm-init")
    }

    pub fn new_command(&self, program: &Path) -> Command {
        let mut command = Command::new(program);

        command.env_clear();
        command.env("PATH", "/usr/bin:/bin:/usr/sbin:/sbin");
        command.env("HOME", self.path());
        if let Some(xdg_data_home) = &self.xdg_data_home {
            command.env("XDG_DATA_HOME", xdg_data_home);
        }

        command
    }

    pub fn dfxvm_as_command_named(&self, filename: &str) -> Command {
        let path = self.path().join(filename);
        if !path.exists() {
            std::fs::copy(dfxvm_path(), &path).unwrap();
            wait_until_file_is_not_busy(&path);
        }
        self.new_command(&path)
    }

    pub fn config_dir(&self) -> PathBuf {
        self.path().join(".config").join("dfx")
    }

    pub fn data_local_dir(&self) -> PathBuf {
        project_dirs::data_local_dir(self.path(), self.xdg_data_home.as_deref())
    }

    pub fn versions_dir(&self) -> PathBuf {
        self.data_local_dir().join("versions")
    }

    pub fn dfx_version_dir(&self, version: &str) -> PathBuf {
        self.versions_dir().join(version)
    }

    pub fn dfx_version_dirs(&self) -> Vec<String> {
        self.versions_dir()
            .read_dir()
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect()
    }

    pub fn installed_dfx_path(&self, version: &str) -> PathBuf {
        self.dfx_version_dir(version).join("dfx")
    }

    pub fn create_executable_dfx_script(&self, version: &str, snippet: &str) -> PathBuf {
        let version = self.dfx_version_dir(version);
        create_dir_all(&version).unwrap();
        let bin_path = version.join("dfx");
        let script = file_contents::bash_script(snippet);
        create_executable(&bin_path, &script);
        bin_path
    }

    pub fn settings(&self) -> Settings {
        Settings::new(self.config_dir().join("version-manager.json"))
    }

    pub fn new_project_temp_dir(&self) -> TempDir {
        tempfile::Builder::new()
            .prefix("integration-test-project")
            .tempdir_in(self.path())
            .unwrap()
    }
}
