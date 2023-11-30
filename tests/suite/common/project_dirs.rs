use std::path::{Path, PathBuf};

#[cfg(target_os = "macos")]
pub fn data_local_dir(home: &Path, _xdg_data_home: Option<&Path>) -> PathBuf {
    home.join("Library")
        .join("Application Support")
        .join("org.dfinity.dfx")
}

#[cfg(target_os = "linux")]
pub fn data_local_dir(home: &Path, xdg_data_home: Option<&Path>) -> PathBuf {
    // See https://github.com/dirs-dev/directories-rs/blob/main/src/lin.rs#L15
    match xdg_data_home {
        Some(xdg_data_home) if xdg_data_home.is_absolute() => xdg_data_home.join("dfx"),
        _ => home.join(".local").join("share").join("dfx"),
    }
}
