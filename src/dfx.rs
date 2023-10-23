use crate::error::dfx;
use crate::error::dfx::Error::Exec;
use crate::error::dfx::{
    DetermineDfxVersionError, FindDfxJsonError, GetVersionFromCommandLineError,
    GetVersionFromDfxJsonError, GetVersionFromEnvironmentError,
};
use crate::error::settings::LoadSettingsError;
use crate::json::load_json_file;
use crate::locations::Locations;
use crate::settings::Settings;
use crate::style::style_command;
use semver::Version;
use serde::Deserialize;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn main(args: &[OsString]) -> Result<ExitCode, dfx::Error> {
    let locations = Locations::new()?;

    let Some((version, args)) = get_dfx_version_and_command_args(args, &locations)? else {
        err!("Unable to determine which dfx version to call. To set a default version, run:");
        err!("    {}", style_command("dfxvm default <version>"));
        return Ok(ExitCode::FAILURE);
    };

    let bin_path = locations.dfx_bin_path(&version);
    if !bin_path.exists() {
        err!("dfx {version} is not installed.  To install it, run:");
        err!("    {}", style_command(&format!("dfxvm install {version}")));
        return Ok(ExitCode::FAILURE);
    }

    let mut command = std::process::Command::new(bin_path);
    command.args(args);
    let err = command.exec();
    Err(Exec {
        command,
        source: err,
    })
}

fn get_dfx_version_and_command_args<'args>(
    args: &'args [OsString],
    locations: &Locations,
) -> Result<Option<(Version, &'args [OsString])>, DetermineDfxVersionError> {
    let args = &args[1..]; // skip the binary name

    if let Some(version) = get_version_from_commandline(args)? {
        Ok(Some((version, &args[1..]))) // skip the version parameter
    } else if let Some(version) = get_version_from_environment()? {
        Ok(Some((version, args)))
    } else if let Some(version) = get_version_from_dfx_json()? {
        Ok(Some((version, args)))
    } else if let Some(version) = get_version_from_settings(locations)? {
        Ok(Some((version, args)))
    } else {
        Ok(None)
    }
}

// the first argument may be a version (starts with "+"),
fn get_version_from_commandline(
    args: &[OsString],
) -> Result<Option<Version>, GetVersionFromCommandLineError> {
    let version = args.first();
    let version = version
        .map(|s| std::str::from_utf8(s.as_bytes()))
        .transpose()?
        .filter(|&s| s.starts_with('+'))
        .map(|s| {
            let v = &s[1..];
            v.parse::<Version>()
                .map_err(|source| GetVersionFromCommandLineError::ParseVersion {
                    version: v.to_string(),
                    source,
                })
        })
        .transpose()?;

    Ok(version)
}

fn get_version_from_environment() -> Result<Option<Version>, GetVersionFromEnvironmentError> {
    std::env::var("DFX_VERSION")
        .ok()
        .map(|version| {
            Version::parse(&version)
                .map_err(|source| GetVersionFromEnvironmentError { version, source })
        })
        .transpose()
}

#[derive(Clone, Debug, Deserialize)]
pub struct DfxJson {
    pub dfx: Option<String>,
}

fn get_version_from_dfx_json() -> Result<Option<Version>, GetVersionFromDfxJsonError> {
    let dfx_json_path: Option<PathBuf> = find_dfx_json()?;
    let Some(dfx_json_path) = dfx_json_path else {
        return Ok(None);
    };
    let dfx_json: DfxJson = load_json_file(&dfx_json_path)?;

    let Some(version) = dfx_json.dfx else {
        return Ok(None);
    };
    let version =
        Version::parse(&version).map_err(|source| GetVersionFromDfxJsonError::ParseVersion {
            version,
            path: dfx_json_path,
            source,
        })?;
    Ok(Some(version))
}

fn find_dfx_json() -> Result<Option<PathBuf>, FindDfxJsonError> {
    let mut maybe_parent = Some(crate::fs::canonicalize(&crate::env::current_dir()?)?);
    while let Some(parent) = maybe_parent {
        let path = parent.join("dfx.json");
        if path.exists() {
            return Ok(Some(path));
        }
        maybe_parent = parent.parent().map(Path::to_path_buf);
    }
    Ok(None)
}

fn get_version_from_settings(locations: &Locations) -> Result<Option<Version>, LoadSettingsError> {
    let path = locations.settings_path();
    if path.exists() {
        let settings = Settings::load(&path)?;

        Ok(Some(settings.default_version))
    } else {
        Ok(None)
    }
}
