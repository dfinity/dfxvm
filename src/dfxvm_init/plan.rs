use crate::installation::{get_detected_profile_scripts, get_env_path_user_facing, ProfileScript};
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
    pub modify_path: bool,
}

impl PlanOptions {
    pub fn new() -> Self {
        Self {
            dfx_version: DfxVersion::Latest,
            modify_path: true,
        }
    }

    pub fn with_dfx_version(self, dfx_version: DfxVersion) -> Self {
        Self {
            dfx_version,
            ..self
        }
    }

    pub fn with_modify_path(self, modify_path: bool) -> Self {
        Self {
            modify_path,
            ..self
        }
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

    pub profile_scripts: Vec<ProfileScript>,
}

impl Plan {
    pub fn new(options: PlanOptions, locations: &Locations) -> Self {
        let bin_dir = locations.bin_dir();
        let env_path = locations.data_local_dir().join("env");
        let env_path_user_facing = get_env_path_user_facing().to_string();
        let profile_scripts = get_detected_profile_scripts();
        Self {
            options,
            bin_dir,
            env_path,
            env_path_user_facing,
            profile_scripts,
        }
    }

    pub fn with_options(self, options: PlanOptions) -> Self {
        Self { options, ..self }
    }
}
