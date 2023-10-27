use crate::errors::dfxvm::SelfUninstallError;
use std::process::ExitCode;

pub fn self_uninstall() -> Result<ExitCode, SelfUninstallError> {
    println!("uninstall dfxvm and all dfx versions");
    Ok(ExitCode::SUCCESS)
}
