use crate::error::dfxvm::update::UpdateError;
use crate::error::{dfxvm::default::DefaultError, dfxvm::install::InstallError};
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
pub enum ListError {}

#[derive(Error, Debug)]
pub enum UninstallError {}

#[derive(Error, Debug)]
pub enum SelfUninstallError {}

#[derive(Error, Debug)]
pub enum SelfUpdateError {}
