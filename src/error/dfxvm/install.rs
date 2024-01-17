use crate::error::{
    download::DownloadVerifiedTarballError,
    fs::{CreateDirAllError, OpenFileError, RenameError},
    json::LoadJsonFileError,
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
