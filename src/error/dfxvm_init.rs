use crate::error::{
    dfxvm,
    env::NoHomeDirectoryError,
    fs::{CreateDirAllError, WriteFileError},
    installation::InstallBinariesError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ExecutePlan(#[from] ExecutePlanError),

    #[error(transparent)]
    Interact(#[from] InteractError),

    #[error(transparent)]
    NoHomeDirectory(#[from] NoHomeDirectoryError),
}

#[derive(Error, Debug)]
pub enum ExecutePlanError {
    #[error(transparent)]
    CreateDirectories(#[from] CreateDirAllError),

    #[error(transparent)]
    SetDefault(#[from] dfxvm::SetDefaultError),

    #[error(transparent)]
    InstallBinaries(#[from] InstallBinariesError),

    #[error(transparent)]
    Update(#[from] dfxvm::UpdateError),

    #[error(transparent)]
    WriteFile(#[from] WriteFileError),
}

#[derive(Error, Debug)]
#[error("failed to interact with console")]
pub struct InteractError {
    #[from]
    source: dialoguer::Error,
}
