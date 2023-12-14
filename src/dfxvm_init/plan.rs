use crate::installation::get_env_path_user_facing;
use crate::locations::Locations;
use semver::Version;
use std::path::PathBuf;

#[derive(Clone)]
pub enum DfxVersion {
    Latest,
    Specific(Version),
}

#[derive(Clone)]
pub struct PlanOptions {
    pub dfx_version: DfxVersion,
}

impl PlanOptions {
    pub fn new() -> Self {
        Self {
            dfx_version: DfxVersion::Latest,
        }
    }

    pub fn with_dfx_version(self, dfx_version: DfxVersion) -> Self {
        Self { dfx_version }
    }
}

pub struct Plan {
    pub options: PlanOptions,

    pub bin_dir: PathBuf,

    pub env_path: PathBuf,

    // This is the path to the env file, but with $HOME or possibly $XDG_DATA_HOME
    // to be replaced by the shell. This is the path that we instruct the user
    // to use with the "source" command, and also the path that we will use when
    // altering profile scripts.
    pub env_path_user_facing: String,
}

impl Plan {
    pub fn new(options: PlanOptions, locations: &Locations) -> Self {
        let bin_dir = locations.data_local_dir().join("bin");
        let env_path = locations.data_local_dir().join("env");
        let env_path_user_facing = get_env_path_user_facing().to_string();
        Self {
            options,
            bin_dir,
            env_path,
            env_path_user_facing,
        }
    }

    pub fn with_options(self, options: PlanOptions) -> Self {
        Self { options, ..self }
    }
}
