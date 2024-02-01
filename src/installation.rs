// "installation" in the noun sense: "the installation"
mod bin;
mod dirs;
mod env;
mod profile;
mod shell;

pub use bin::install_binaries;
pub use env::{env_file_contents, get_env_path_user_facing};
pub use profile::ProfileScript;
pub use shell::{get_all_profile_scripts, get_detected_profile_scripts};
