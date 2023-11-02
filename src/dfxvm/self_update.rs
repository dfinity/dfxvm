use crate::error::dfxvm::SelfUpdateError;
use std::process::ExitCode;

pub fn self_update() -> Result<ExitCode, SelfUpdateError> {
    println!("update dfxvm to latest");
    Ok(ExitCode::SUCCESS)
}
