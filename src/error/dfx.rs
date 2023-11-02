use crate::error::env::{GetCurrentDirError, NoHomeDirectoryError};
use crate::error::fs::CanonicalizePathError;
use crate::error::json::LoadJsonFileError;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DetermineDfxVersion(#[from] DetermineDfxVersionError),

    #[error("failed to execute {command:#?}")]
    Exec {
        command: Command,
        source: std::io::Error,
    },

    #[error(transparent)]
    NoHomeDirectory(#[from] NoHomeDirectoryError),
}

#[derive(Error, Debug)]
pub enum DetermineDfxVersionError {
    #[error(transparent)]
    GetVersionFromEnvironment(#[from] GetVersionFromEnvironmentError),

    #[error(transparent)]
    GetVersionFromDfxJson(#[from] GetVersionFromDfxJsonError),

    #[error(transparent)]
    LoadSettings(#[from] LoadJsonFileError),

    #[error(transparent)]
    GetVersionFromCommandLine(#[from] GetVersionFromCommandLineError),
}

#[derive(Error, Debug)]
pub enum GetVersionFromCommandLineError {
    #[error("failed to parse version from commandline")]
    InvalidUtf8(#[from] std::str::Utf8Error),

    #[error("failed to parse version '{version}' from commandline")]
    ParseVersion {
        version: String,
        source: semver::Error,
    },
}

#[derive(Error, Debug)]
#[error("failed to parse DFX_VERSION '{version}' from environment")]
pub struct GetVersionFromEnvironmentError {
    pub version: String,
    pub source: semver::Error,
}

#[derive(Error, Debug)]
pub enum GetVersionFromDfxJsonError {
    #[error(transparent)]
    FindDfxJson(#[from] FindDfxJsonError),

    #[error(transparent)]
    LoadDfxJson(#[from] LoadJsonFileError),
}

#[derive(Error, Debug)]
pub enum FindDfxJsonError {
    #[error(transparent)]
    GetCurrentDir(#[from] GetCurrentDirError),

    #[error(transparent)]
    CanonicalizePath(#[from] CanonicalizePathError),
}
