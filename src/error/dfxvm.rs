use crate::error::{
    dfxvm::self_update::CleanupSelfUpdaterError,
    fs::{RemoveDirAllError, RemoveFileError, RenameError},
    json::{FetchJsonDocError, LoadJsonFileError},
};
use std::path::PathBuf;
use thiserror::Error;

pub mod default;
pub mod install;
pub mod self_uninstall;
pub mod self_update;

pub use default::DefaultError;
pub use default::SetDefaultError;
pub use install::InstallError;
pub use self_uninstall::SelfUninstallError;
pub use self_update::SelfUpdateError;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CleanupSelfUpdater(#[from] CleanupSelfUpdaterError),

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

    #[error("failed to read directory {path}")]
    ReadDir {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum UninstallError {
    #[error(transparent)]
    RemoveDirAll(#[from] RemoveDirAllError),

    #[error(transparent)]
    RemoveFile(#[from] RemoveFileError),

    #[error(transparent)]
    Rename(#[from] RenameError),
}

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("failed to fetch latest tag")]
    FetchLatestTag(#[from] FetchJsonDocError),

    #[error(transparent)]
    LoadSettings(#[from] LoadJsonFileError),

    #[error("failed to parse manifest url")]
    ParseManifestUrl(#[from] url::ParseError),

    #[error(transparent)]
    SetDefault(#[from] SetDefaultError),
}
