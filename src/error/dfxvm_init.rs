use crate::error::{
    dfxvm,
    dfxvm::self_update::SelfReplaceError,
    fs::{AppendToFileError, CreateDirAllError, ReadToStringError, WriteFileError},
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
    SelfReplace(#[from] SelfReplaceError),
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
    UpdateProfileScripts(#[from] UpdateProfileScriptsError),

    #[error(transparent)]
    WriteFile(#[from] WriteFileError),
}

#[derive(Error, Debug)]
#[error("failed to interact with console")]
pub struct InteractError {
    #[from]
    source: dialoguer::Error,
}

#[derive(Error, Debug)]
pub enum UpdateProfileScriptsError {
    #[error(transparent)]
    AppendToFile(#[from] AppendToFileError),

    #[error(transparent)]
    ReadProfileScript(#[from] ReadToStringError),
}
