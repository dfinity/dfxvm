use crate::common::dfxvm_path;
use std::process::Command;
use tempdir::TempDir;

pub struct TempHomeDir {
    tempdir: TempDir,
}
impl TempHomeDir {
    pub fn new() -> Self {
        let tempdir = TempDir::new("dfxvm-integration-tests-home").unwrap();
        Self { tempdir }
    }

    pub fn dfx(&self) -> Command {
        self.command("dfx")
    }

    pub fn dfxvm(&self) -> Command {
        self.command("dfxvm")
    }

    pub fn dfxvm_init(&self) -> Command {
        self.command("dfxvm-init")
    }

    pub fn command(&self, filename: &str) -> Command {
        let path = self.tempdir.path().join(filename);
        std::fs::copy(dfxvm_path(), &path).unwrap();
        let mut command = Command::new(path);
        command.env("HOME", self.tempdir.path());
        command
    }
}
