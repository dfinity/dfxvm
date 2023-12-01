use crate::installation::get_env_path_user_facing;
use crate::locations::Locations;
use semver::Version;
use std::path::PathBuf;

#[derive(Clone)]
pub enum DfxVersion {
    Latest,
    Specific(Version),
}

pub struct Plan {
    pub bin_dir: PathBuf,
    pub dfx_version: DfxVersion,

    pub env_path: PathBuf,

    // This is the path to the env file, but with $HOME or possibly $XDG_DATA_HOME
    // to be replaced by the shell. This is the path that we instruct the user
    // to use with the "source" command, and also the path that we will use when
    // altering profile scripts.
    pub env_path_user_facing: String,
}

impl Plan {
    pub fn new(locations: &Locations) -> Self {
        let bin_dir = locations.data_local_dir().join("bin");
        let env_path = locations.data_local_dir().join("env");
        let dfx_version = DfxVersion::Latest;
        let env_path_user_facing = get_env_path_user_facing().to_string();
        Self {
            bin_dir,
            env_path,
            env_path_user_facing,
            dfx_version,
        }
    }

    pub fn with_dfx_version(self, dfx_version: DfxVersion) -> Self {
        Self {
            dfx_version,
            ..self
        }
    }
}
