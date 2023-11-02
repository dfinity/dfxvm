use crate::error::fs::ReadFileError;
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
