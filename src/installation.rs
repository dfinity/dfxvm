// "installation" in the noun sense: "the installation"
mod bin;
mod env;

pub use bin::install_binaries;
pub use env::{env_file_contents, get_env_path_user_facing};
