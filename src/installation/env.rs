#[cfg(target_os = "macos")]
pub fn get_env_path_user_facing() -> &'static str {
    "$HOME/Library/Application Support/org.dfinity.dfx/env"
}

#[cfg(target_os = "linux")]
pub fn get_env_path_user_facing() -> &'static str {
    if crate::env::use_xdg_data_home() {
        "$XDG_DATA_HOME/dfx/env"
    } else {
        "$HOME/.local/share/dfx/env"
    }
}

#[cfg(target_os = "macos")]
pub fn env_file_contents() -> &'static str {
    include_str!("env/macos.sh")
}

#[cfg(target_os = "linux")]
pub fn env_file_contents() -> &'static str {
    if crate::env::use_xdg_data_home() {
        include_str!("env/linux-xdg-data-home.sh")
    } else {
        include_str!("env/linux-home.sh")
    }
}
