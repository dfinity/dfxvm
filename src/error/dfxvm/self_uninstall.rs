use crate::error::{
    dfxvm_init::InteractError,
    fs::{ReadFileError, RemoveDirAllError, RemoveFileError, WriteFileError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SelfUninstallError {
    #[error(transparent)]
    Interact(#[from] InteractError),

    #[error(transparent)]
    RemoveDirAll(#[from] RemoveDirAllError),

    #[error(transparent)]
    RemoveFile(#[from] RemoveFileError),

    #[error(transparent)]
    UninstallProfileScripts(#[from] UninstallProfileScriptsError),
}

#[derive(Error, Debug)]
pub enum UninstallProfileScriptsError {
    #[error(transparent)]
    ReadFile(#[from] ReadFileError),

    #[error(transparent)]
    WriteFile(#[from] WriteFileError),
}
