use crate::error::json::LoadJsonFileError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadSettingsError {
    #[error(transparent)]
    LoadJsonFile(#[from] LoadJsonFileError),

    #[error("failed to parse default version '{version}' from {path}")]
    ParseDefaultVersion {
        version: String,
        path: PathBuf,
        source: semver::Error,
    },
}
