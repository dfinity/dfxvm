use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("failed to canonicalize path '{path}'")]
pub struct CanonicalizePathError {
    pub path: PathBuf,
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to read {path}")]
pub struct ReadFileError {
    pub path: PathBuf,
    pub source: std::io::Error,
}
