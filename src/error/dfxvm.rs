use crate::error::{
    dfxvm::{default::DefaultError, install::InstallError, update::UpdateError},
    env::NoHomeDirectoryError,
    fs::{RemoveDirAllError, RemoveFileError, RenameError},
    json::LoadJsonFileError,
};
use std::path::PathBuf;
use thiserror::Error;

pub mod default;
pub mod install;
pub mod update;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Default(#[from] DefaultError),

    #[error(transparent)]
    Install(#[from] InstallError),

    #[error(transparent)]
    List(#[from] ListError),

    #[error(transparent)]
    SelfUninstall(#[from] SelfUninstallError),

    #[error(transparent)]
    SelfUpdate(#[from] SelfUpdateError),

    #[error(transparent)]
    Uninstall(#[from] UninstallError),

    #[error(transparent)]
    Update(#[from] UpdateError),
}

#[derive(Error, Debug)]
pub enum ListError {
    #[error(transparent)]
    LoadJsonFile(#[from] LoadJsonFileError),

    #[error(transparent)]
    NoHomeDirectory(#[from] NoHomeDirectoryError),

    #[error("failed to read directory {path}")]
    ReadDir {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum UninstallError {
    #[error(transparent)]
    NoHomeDirectory(#[from] NoHomeDirectoryError),

    #[error(transparent)]
    RemoveDirAll(#[from] RemoveDirAllError),

    #[error(transparent)]
    RemoveFile(#[from] RemoveFileError),

    #[error(transparent)]
    Rename(#[from] RenameError),
}

#[derive(Error, Debug)]
pub enum SelfUninstallError {}

#[derive(Error, Debug)]
pub enum SelfUpdateError {}
