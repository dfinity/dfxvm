use crate::errors::dfxvm::UninstallError;
use semver::Version;
use std::process::ExitCode;

pub fn uninstall(version: Version) -> Result<ExitCode, UninstallError> {
    println!("uninstall dfx {}", version);
    Ok(ExitCode::SUCCESS)
}
