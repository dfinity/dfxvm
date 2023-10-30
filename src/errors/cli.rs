use crate::errors::dfxvm;
use crate::errors::{dfx, dfxvm_init};
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
    Proxy(#[from] dfx::Error),
}

#[derive(Error, Debug)]
pub enum DetermineModeError {
    #[error("Couldn't determine self executable name")]
    NoExeName,

    #[error("Unrecognized executable name '{0}'. Expect one of: dfx, dfxvm, dfxvm-init")]
    UnrecognizedExeName(String),
}
