use crate::error::{
    download::{DownloadFileError, VerifyChecksumError},
    env::NoHomeDirectoryError,
    fs::{OpenFileError, RemoveFileError},
    installation::InstallBinariesError,
    json::{FetchJsonDocError, LoadJsonFileError},
};
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum SelfUpdateError {
    #[error(transparent)]
    DownloadLatestBinaryError(#[from] DownloadLatestBinaryError),

    #[error("failed to execute {command:#?}")]
    Exec {
        command: Command,
        source: std::io::Error,
    },

    #[error(transparent)]
    LoadJsonFile(#[from] LoadJsonFileError),

    #[error(transparent)]
    LookupLatestVersionError(#[from] LookupLatestVersionError),

    #[error(transparent)]
    FormatTarballUrl(#[from] FormatTarballUrlError),
}

#[derive(Error, Debug)]
#[error("failed to format tarball url {url}")]
pub struct FormatTarballUrlError {
    pub url: String,
    pub source: url::ParseError,
}

#[derive(Error, Debug)]
pub enum LookupLatestVersionError {
    #[error(transparent)]
    FetchJsonDoc(#[from] FetchJsonDocError),

    #[error("failed to parse url {url}")]
    ParseUrl {
        url: String,
        source: url::ParseError,
    },

    #[error("no dfxvm release found at {url}")]
    NoDfxvmRelease { url: Url },
}

#[derive(Error, Debug)]
pub enum DownloadLatestBinaryError {
    #[error("failed to create a temporary directory in {path}")]
    CreateTempDirIn {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error(transparent)]
    DownloadFile(#[from] DownloadFileError),

    #[error(transparent)]
    ExtractBinary(#[from] ExtractBinaryError),

    #[error(transparent)]
    ParseUrl(#[from] url::ParseError),

    #[error(transparent)]
    VerifyChecksum(#[from] VerifyChecksumError),
}

#[derive(Error, Debug)]
pub enum ExtractBinaryError {
    #[error("dfxvm not found in archive")]
    DfxvmNotFound,

    #[error(transparent)]
    OpenFile(#[from] OpenFileError),

    #[error("failed to read archive entries")]
    ReadArchiveEntries(#[source] std::io::Error),

    #[error("failed to unpack binary")]
    UnpackBinary(#[source] std::io::Error),
}

#[derive(Error, Debug)]
pub enum SelfReplaceError {
    #[error(transparent)]
    InstallBinaries(#[from] InstallBinariesError),

    #[error(transparent)]
    NoHomeDirectory(#[from] NoHomeDirectoryError),
}

#[derive(Error, Debug)]
pub enum CleanupSelfUpdaterError {
    #[error(transparent)]
    RemoveFile(#[from] RemoveFileError),
}
