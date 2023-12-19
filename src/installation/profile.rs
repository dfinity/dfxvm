use crate::installation::{get_env_path_user_facing, profile::ProfileScriptType::Posix};
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProfileScriptType {
    Posix,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileScript {
    pub path: PathBuf,
    pub script_type: ProfileScriptType,
}

impl ProfileScript {
    pub fn posix(path: PathBuf) -> Self {
        Self {
            path,
            script_type: Posix,
        }
    }

    pub fn source_string(&self) -> String {
        // see https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#dot
        format!(r#". "{}""#, get_env_path_user_facing())
    }

    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }
}
