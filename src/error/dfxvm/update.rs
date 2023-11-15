use crate::error::dfxvm::default::SetDefaultError;
use crate::error::env::NoHomeDirectoryError;
use crate::error::json::LoadJsonFileError;
use crate::error::reqwest::WrappedReqwestError;
use crate::error::Retryable;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("failed to fetch latest tag")]
    FetchLatestTag(#[from] FetchManifestError),

    #[error(transparent)]
    LoadSettings(#[from] LoadJsonFileError),

    #[error(transparent)]
    NoHomeDirectory(#[from] NoHomeDirectoryError),

    #[error("failed to parse manifest url")]
    ParseManifestUrl(#[from] url::ParseError),

    #[error(transparent)]
    SetDefault(#[from] SetDefaultError),
}

#[derive(Error, Debug)]
pub enum FetchManifestError {
    #[error(transparent)]
    Get(WrappedReqwestError),

    #[error(transparent)]
    Status(WrappedReqwestError),

    #[error(transparent)]
    ReadBytes(WrappedReqwestError),

    #[error("failed to parse manifest.json")]
    Parse(serde_json::Error),
}

impl Retryable for FetchManifestError {
    fn is_retryable(&self) -> bool {
        match self {
            FetchManifestError::Get(e) => e.is_retryable(),
            FetchManifestError::ReadBytes(_) => true,
            _ => false,
        }
    }
}
