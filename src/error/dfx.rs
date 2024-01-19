use crate::error::{
    dfxvm::self_update::CleanupSelfUpdaterError, env::GetCurrentDirError,
    fs::CanonicalizePathError, json::LoadJsonFileError,
};
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CleanupSelfUpdater(#[from] CleanupSelfUpdaterError),

    #[error(transparent)]
    DetermineDfxVersion(#[from] DetermineDfxVersionError),

    #[error("failed to execute {command:#?}")]
    Exec {
        command: Command,
        source: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum DetermineDfxVersionError {
    #[error(transparent)]
    GetVersionFromCommandLine(#[from] GetVersionFromCommandLineError),

    #[error(transparent)]
    GetVersionFromDfxJson(#[from] GetVersionFromDfxJsonError),

    #[error(transparent)]
    GetVersionFromEnvironment(#[from] GetVersionFromEnvironmentError),

    #[error(transparent)]
    LoadSettings(#[from] LoadJsonFileError),
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
    CanonicalizePath(#[from] CanonicalizePathError),

    #[error(transparent)]
    GetCurrentDir(#[from] GetCurrentDirError),
}
