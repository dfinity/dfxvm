use crate::errors::dfxvm::ListError;
use std::process::ExitCode;

pub fn list() -> Result<ExitCode, ListError> {
    println!("list installed dfx versions");
    Ok(ExitCode::SUCCESS)
}
