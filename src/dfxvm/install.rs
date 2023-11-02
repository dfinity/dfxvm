use crate::error::dfxvm::InstallError;
use semver::Version;
use std::process::ExitCode;

pub fn install(version: Version) -> Result<ExitCode, InstallError> {
    println!("install dfx {}", version);
    Ok(ExitCode::SUCCESS)
}
