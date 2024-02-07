use crate::dfxvm::cleanup_self_updater;
use crate::env::prepend_to_path;
use crate::error::dfx;
use crate::error::dfx::Error::Exec;
use crate::error::dfx::{
    DetermineDfxVersionError, FindDfxJsonError, GetVersionFromCommandLineError,
    GetVersionFromDfxJsonError, GetVersionFromEnvironmentError,
};
use crate::error::json::LoadJsonFileError;
use crate::json::load_json_file;
use crate::locations::Locations;
use crate::settings::Settings;
use crate::style::style_command;
use semver::Version;
use serde::Deserialize;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn main(args: &[OsString], locations: &Locations) -> Result<ExitCode, dfx::Error> {
    if trying_to_call_dfx_upgrade(args) {
        err!("The command `dfx upgrade` doesn't work with dfxvm.");
        err!("To upgrade dfx, run:");
        err!("    {}", style_command("dfxvm update"));
        return Ok(ExitCode::FAILURE);
    }
    cleanup_self_updater(locations)?;
    let Some((version, args)) = get_dfx_version_and_command_args(args, locations)? else {
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
    command.env("DFX_VERSION", version.to_string());
    command.env("PATH", prepend_to_path(&locations.version_dir(&version)));
    let err = command.exec();
    Err(Exec {
        command,
        source: err,
    })
}

fn trying_to_call_dfx_upgrade(args: &[OsString]) -> bool {
    let mut iter = args.iter().peekable();

    // Skip the first argument (binary name)
    iter.next();

    let mut skip_next = false;

    for arg in iter {
        if skip_next {
            // Skip the next argument if it's a parameter value
            skip_next = false;
            continue;
        }

        // Convert the argument to a string
        if let Some(arg_str) = arg.to_str() {
            if arg_str.starts_with('-') {
                if arg_str == "--identity" || arg_str == "--network" || arg_str == "--logfile" {
                    // The next parameter is an argument, so skip it
                    skip_next = true;
                }
            } else {
                // Check if the first non-parameter "word" is "upgrade"
                return arg_str == "upgrade";
            }
        }
    }
    false
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
        .filter(|s| !s.trim().is_empty())
        .map(|version| {
            Version::parse(&version)
                .map_err(|source| GetVersionFromEnvironmentError { version, source })
        })
        .transpose()
}

#[derive(Clone, Debug, Deserialize)]
pub struct DfxJson {
    pub dfx: Option<Version>,
}

fn get_version_from_dfx_json() -> Result<Option<Version>, GetVersionFromDfxJsonError> {
    let dfx_json_path: Option<PathBuf> = find_dfx_json()?;
    let Some(dfx_json_path) = dfx_json_path else {
        return Ok(None);
    };
    let dfx_json: DfxJson = load_json_file(&dfx_json_path)?;

    Ok(dfx_json.dfx)
}

fn find_dfx_json() -> Result<Option<PathBuf>, FindDfxJsonError> {
    for parent in crate::fs::canonicalize(&crate::env::current_dir()?)?.ancestors() {
        let path = parent.join("dfx.json");
        if path.exists() {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

fn get_version_from_settings(locations: &Locations) -> Result<Option<Version>, LoadJsonFileError> {
    let path = locations.settings_path();
    Settings::load_or_default(&path).map(|settings| settings.default_version)
}
