use crate::errors::dfxvm::DefaultError;
use semver::Version;
use std::process::ExitCode;

pub fn default(version: Version) -> Result<ExitCode, DefaultError> {
    println!("use dfx {} by default", version);
    Ok(ExitCode::SUCCESS)
}
