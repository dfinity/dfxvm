#[cfg(target_os = "macos")]
pub fn get_data_local_dir_user_facing() -> &'static str {
    "$HOME/Library/Application Support/org.dfinity.dfx"
}

#[cfg(target_os = "linux")]
pub fn get_data_local_dir_user_facing() -> &'static str {
    if crate::env::use_xdg_data_home() {
        "$XDG_DATA_HOME/dfx"
    } else {
        "$HOME/.local/share/dfx"
    }
}

pub fn get_bin_dir_user_facing() -> String {
    format!("{}/bin", get_data_local_dir_user_facing())
}
