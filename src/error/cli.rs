use crate::error::{dfx, dfxvm, dfxvm_init, env::NoHomeDirectoryError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DispatchError {
    #[error(transparent)]
    DetermineMode(#[from] DetermineModeError),

    #[error(transparent)]
    Init(#[from] dfxvm_init::Error),

    #[error(transparent)]
    Manage(#[from] dfxvm::Error),

    #[error(transparent)]
    NoHomeDirectory(#[from] NoHomeDirectoryError),

    #[error(transparent)]
    Proxy(#[from] dfx::Error),
}

#[derive(Error, Debug)]
pub enum DetermineModeError {
    #[error("couldn't determine self executable name")]
    NoExeName,

    #[error("unrecognized executable name '{0}'; expect one of: dfx, dfxvm, dfxvm-init")]
    UnrecognizedExeName(String),
}
