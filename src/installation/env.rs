use crate::installation::dirs::{get_bin_dir_user_facing, get_data_local_dir_user_facing};

const ENV_FILE_TEMPLATE: &str = include_str!("env.sh");

pub fn get_env_path_user_facing() -> String {
    format!("{}/env", get_data_local_dir_user_facing())
}

pub fn env_file_contents() -> String {
    ENV_FILE_TEMPLATE
        .to_string()
        .replace("{dfxvm_bin}", &get_bin_dir_user_facing())
}
