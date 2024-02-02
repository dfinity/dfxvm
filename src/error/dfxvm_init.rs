use crate::error::{
    dfxvm,
    dfxvm::self_update::SelfReplaceError,
    fs::{
        AppendToFileError, CanonicalizePathError, CreateDirAllError, ReadToStringError,
        RemoveFileError, WriteFileError,
    },
    installation::InstallBinariesError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CanonicalizePath(#[from] CanonicalizePathError),

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
    DeleteDfxOnPath(#[from] DeleteDfxOnPathError),

    #[error(transparent)]
    SetDefault(#[from] dfxvm::SetDefaultError),

    #[error(transparent)]
    InstallBinaries(#[from] InstallBinariesError),

    #[error(transparent)]
    RemoveFile(#[from] RemoveFileError),

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

#[derive(Error, Debug)]
pub enum DeleteDfxOnPathError {
    #[error(transparent)]
    Interact(#[from] InteractError),

    #[error("failed to call sudo rm")]
    CallSudoRm(#[from] std::io::Error),
}
