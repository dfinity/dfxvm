use crate::error::{
    env::NoHomeDirectoryError,
    fs::{
        CreateDirAllError, CreateFileError, OpenFileError, ReadToStringError, RenameError,
        WriteFileError,
    },
    json::LoadJsonFileError,
    reqwest::WrappedReqwestError,
    Retryable,
};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstallError {
    #[error("failed to create a temporary directory")]
    CreateTempDir(#[source] std::io::Error),

    #[error("failed to create a temporary directory in {path}")]
    CreateTempDirIn {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error(transparent)]
    CreateVersionsDir(#[from] CreateDirAllError),

    #[error(transparent)]
    DownloadVerifiedTarball(#[from] DownloadVerifiedTarballError),

    #[error(transparent)]
    ExtractArchive(#[from] ExtractArchiveError),

    #[error(transparent)]
    InstallVersionDirectory(#[from] RenameError),

    #[error(transparent)]
    LoadSettings(#[from] LoadJsonFileError),

    #[error(transparent)]
    NoHomeDirectory(#[from] NoHomeDirectoryError),
}

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
pub enum ExtractArchiveError {
    #[error(transparent)]
    OpenFile(#[from] OpenFileError),

    #[error("failed to unpack archive {path}")]
    Unpack {
        path: PathBuf,
        source: std::io::Error,
    },
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
