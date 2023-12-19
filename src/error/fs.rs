use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppendToFileError {
    #[error(transparent)]
    Open(#[from] OpenFileError),

    #[error(transparent)]
    Write(#[from] WriteFileError),

    #[error(transparent)]
    Sync(#[from] SyncDataError),
}

#[derive(Error, Debug)]
#[error("failed to canonicalize '{path}'")]
pub struct CanonicalizePathError {
    pub path: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to copy {from} to {to}")]
pub struct CopyFileError {
    pub from: PathBuf,
    pub to: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to create directory {path} and parents")]
pub struct CreateDirAllError {
    pub path: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to create {path}")]
pub struct CreateFileError {
    pub path: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to open {path}")]
pub struct OpenFileError {
    pub path: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to read {path}")]
pub struct ReadFileError {
    pub path: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to read metadata for {path}")]
pub struct ReadMetadataError {
    pub path: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to read {path} as string")]
pub struct ReadToStringError {
    pub path: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to remove directory {path} and contents")]
pub struct RemoveDirAllError {
    pub path: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to remove {path}")]
pub struct RemoveFileError {
    pub path: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to rename {from} to {to}")]
pub struct RenameError {
    pub from: PathBuf,
    pub to: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to set permissions for {path}")]
pub struct SetPermissionsError {
    pub path: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to sync data for {path}")]
pub struct SyncDataError {
    pub path: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to write {path}")]
pub struct WriteFileError {
    pub path: PathBuf,
    pub source: std::io::Error,
}
