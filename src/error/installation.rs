use crate::error::env::GetCurrentExeError;
use crate::error::fs::{
    CopyFileError, ReadMetadataError, RemoveFileError, RenameError, SetPermissionsError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstallBinariesError {
    #[error(transparent)]
    CopyFile(#[from] CopyFileError),

    #[error(transparent)]
    GetCurrentExe(#[from] GetCurrentExeError),

    #[error(transparent)]
    MakeExecutable(#[from] MakeExecutableError),

    #[error(transparent)]
    RemoveFile(#[from] RemoveFileError),

    #[error(transparent)]
    Rename(#[from] RenameError),
}

#[derive(Error, Debug)]
pub enum MakeExecutableError {
    #[error(transparent)]
    ReadMetadata(#[from] ReadMetadataError),

    #[error(transparent)]
    SetPermissions(#[from] SetPermissionsError),
}
