use crate::error::{
    dfxvm::install::InstallError,
    fs::CreateDirAllError,
    json::{LoadJsonFileError, SaveJsonFileError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DefaultError {
    #[error(transparent)]
    Display(#[from] DisplayDefaultError),

    #[error(transparent)]
    Set(#[from] SetDefaultError),
}

#[derive(Error, Debug)]
pub enum SetDefaultError {
    #[error(transparent)]
    CreateDirAll(#[from] CreateDirAllError),

    #[error(transparent)]
    Install(#[from] InstallError),

    #[error(transparent)]
    LoadJsonFile(#[from] LoadJsonFileError),

    #[error(transparent)]
    SaveJsonFile(#[from] SaveJsonFileError),
}

#[derive(Error, Debug)]
pub enum DisplayDefaultError {
    #[error(transparent)]
    LoadJsonFile(#[from] LoadJsonFileError),

    #[error("no default dfx version configured")]
    NoDefaultVersion,
}
