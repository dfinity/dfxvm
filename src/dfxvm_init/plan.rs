use crate::error::fs::CanonicalizePathError;
use crate::fs::canonicalize;
use crate::installation::{get_detected_profile_scripts, get_env_path_user_facing, ProfileScript};
use crate::locations::Locations;
use semver::Version;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub enum DfxVersion {
    Latest,
    Specific(Version),
}

#[derive(Clone)]
pub struct PlanOptions {
    pub dfx_version: DfxVersion,
    pub modify_path: bool,
    pub delete_dfx_on_path: bool,
}

impl PlanOptions {
    pub fn new() -> Self {
        Self {
            dfx_version: DfxVersion::Latest,
            modify_path: true,
            delete_dfx_on_path: true,
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

    pub fn delete_dfx_on_path(self, delete_dfx_on_path: bool) -> Self {
        Self {
            delete_dfx_on_path,
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

    pub dfx_on_path: Vec<PathBuf>,
    pub profile_scripts: Vec<ProfileScript>,
}

impl Plan {
    pub fn new(options: PlanOptions, locations: &Locations) -> Result<Self, CanonicalizePathError> {
        let bin_dir = locations.bin_dir();
        let env_path = locations.data_local_dir().join("env");
        let env_path_user_facing = get_env_path_user_facing().to_string();
        let profile_scripts = get_detected_profile_scripts();
        let dfx_on_path = legacy_binaries(&locations.dfx_proxy_path())?;

        Ok(Self {
            options,
            bin_dir,
            env_path,
            env_path_user_facing,
            dfx_on_path,
            profile_scripts,
        })
    }

    pub fn with_options(self, options: PlanOptions) -> Self {
        Self { options, ..self }
    }
}

// find dfx binaries on PATH, excluding the dfx proxy
fn legacy_binaries(dfx_proxy: &Path) -> Result<Vec<PathBuf>, CanonicalizePathError> {
    let all_dfx_on_path = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
        .filter_map(|dir| {
            let dfx_on_path = dir.join("dfx");
            dfx_on_path.exists().then(|| canonicalize(&dfx_on_path))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let legacy_binaries = if dfx_proxy.exists() {
        let canonical_dfx_proxy = canonicalize(dfx_proxy)?;
        all_dfx_on_path
            .into_iter()
            .filter(|p| *p != canonical_dfx_proxy)
            .collect()
    } else {
        all_dfx_on_path
    };
    Ok(legacy_binaries)
}
