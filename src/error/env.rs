use thiserror::Error;

#[derive(Error, Debug)]
#[error("failed to get current directory")]
pub struct GetCurrentDirError {
    #[from]
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("failed to get path of current executable")]
pub struct GetCurrentExeError {
    #[from]
    pub source: std::io::Error,
}

#[derive(Error, Debug)]
#[error("no HOME in environment")]
pub struct NoHomeDirectoryError;
