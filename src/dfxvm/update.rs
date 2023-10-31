use crate::errors::dfxvm::UpdateError;
use std::process::ExitCode;

pub fn update() -> Result<ExitCode, UpdateError> {
    println!("update to latest dfx");
    Ok(ExitCode::SUCCESS)
}
