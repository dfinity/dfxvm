use crate::error::env::{GetCurrentDirError, GetCurrentExeError, NoHomeDirectoryError};
use std::path::PathBuf;

pub fn current_dir() -> Result<PathBuf, GetCurrentDirError> {
    Ok(std::env::current_dir()?)
}

pub fn current_exe() -> Result<PathBuf, GetCurrentExeError> {
    Ok(std::env::current_exe()?)
}

pub fn home_dir() -> Result<PathBuf, NoHomeDirectoryError> {
    #[cfg(unix)]
    let home = std::env::var_os("HOME").ok_or(NoHomeDirectoryError)?;
    Ok(PathBuf::from(home))
}

#[cfg(target_os = "linux")]
pub fn use_xdg_data_home() -> bool {
    // See https://github.com/dirs-dev/directories-rs/blob/main/src/lin.rs#L15
    matches!(std::env::var_os("XDG_DATA_HOME"), Some(p) if PathBuf::from(&p).is_absolute())
}
