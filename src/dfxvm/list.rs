use crate::error::dfxvm::ListError;
use std::process::ExitCode;

pub fn list() -> Result<ExitCode, ListError> {
    println!("list installed dfx versions");
    Ok(ExitCode::SUCCESS)
}
