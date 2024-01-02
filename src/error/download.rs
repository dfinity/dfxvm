use crate::error::{
    fs::{CreateFileError, ReadToStringError, WriteFileError},
    reqwest::WrappedReqwestError,
    Retryable,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadVerifiedTarballError {
    #[error(transparent)]
    DownloadFile(#[from] DownloadFileError),

    #[error("no such version")]
    NoSuchVersion(#[source] WrappedReqwestError),

    #[error("failed to parse url")]
    ParseUrl(#[from] url::ParseError),

    #[error(transparent)]
    VerifyChecksum(#[from] VerifyChecksumError),
}

#[derive(Error, Debug)]
pub enum DownloadFileError {
    #[error(transparent)]
    CreateFile(#[from] CreateFileError),

    #[error("failed to download contents of {url}")]
    DownloadContents {
        url: String,
        source: WrappedReqwestError,
    },

    #[error(transparent)]
    Get(WrappedReqwestError),

    #[error("failed to get content length from {url}")]
    GetContentLength { url: String },

    #[error(transparent)]
    Status(WrappedReqwestError),

    #[error(transparent)]
    WriteFile(#[from] WriteFileError),
}

impl Retryable for DownloadFileError {
    fn is_retryable(&self) -> bool {
        match self {
            DownloadFileError::DownloadContents { .. } => true,
            DownloadFileError::Get(e) => e.is_retryable(),
            _ => false,
        }
    }
}

#[derive(Error, Debug)]
pub enum VerifyChecksumError {
    #[error("checksum did not match.  Expected={expected} Actual={actual}")]
    HashMismatch { expected: String, actual: String },

    #[error("checksum file is malformed. Expected the first word of the first line to contain a hash.  Actual contents: {contents}")]
    MalformedChecksumFile { contents: String },

    #[error(transparent)]
    ReadToString(#[from] ReadToStringError),
}
