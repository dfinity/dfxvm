use crate::error::env::{GetCurrentDirError, NoHomeDirectoryError};
use std::path::PathBuf;

pub fn current_dir() -> Result<PathBuf, GetCurrentDirError> {
    Ok(std::env::current_dir()?)
}

pub fn home_dir() -> Result<PathBuf, NoHomeDirectoryError> {
    #[cfg(unix)]
    let home = std::env::var_os("HOME").ok_or(NoHomeDirectoryError)?;
    Ok(PathBuf::from(home))
}
