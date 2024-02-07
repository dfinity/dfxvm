use crate::error::env::{GetCurrentDirError, GetCurrentExeError, NoHomeDirectoryError};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
const PATH_ENV_SEPARATOR: &str = ";";
#[cfg(unix)]
const PATH_ENV_SEPARATOR: &str = ":";

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

pub fn prepend_to_path(dir_to_prepend: &Path) -> OsString {
    let path = std::env::var_os("PATH").unwrap_or_default();

    let mut path_with_dfx_version_dir =
        OsString::with_capacity(dir_to_prepend.as_os_str().len() + 1 + path.as_os_str().len());
    path_with_dfx_version_dir.push(dir_to_prepend.as_os_str());
    path_with_dfx_version_dir.push(PATH_ENV_SEPARATOR);
    path_with_dfx_version_dir.push(path.as_os_str());
    path_with_dfx_version_dir
}
