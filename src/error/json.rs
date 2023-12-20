use crate::error::{
    fs::{ReadFileError, WriteFileError},
    reqwest::WrappedReqwestError,
    Retryable,
};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadJsonFileError {
    #[error("failed to parse {path} as json")]
    Parse {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error(transparent)]
    Read(#[from] ReadFileError),
}

#[derive(Error, Debug)]
pub enum SaveJsonFileError {
    #[error("failed to serialize json for {path}")]
    Serialize {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error(transparent)]
    Write(#[from] WriteFileError),
}

#[derive(Error, Debug)]
pub enum FetchJsonDocError {
    #[error(transparent)]
    Get(WrappedReqwestError),

    #[error(transparent)]
    Status(WrappedReqwestError),

    #[error(transparent)]
    ReadBytes(WrappedReqwestError),

    #[error("failed to parse json document")]
    Parse(serde_json::Error),
}

impl Retryable for FetchJsonDocError {
    fn is_retryable(&self) -> bool {
        match self {
            FetchJsonDocError::Get(e) => e.is_retryable(),
            FetchJsonDocError::ReadBytes(_) => true,
            _ => false,
        }
    }
}
