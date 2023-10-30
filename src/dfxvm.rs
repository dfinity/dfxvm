use crate::errors::dfxvm;
use std::ffi::OsString;
use std::process::ExitCode;

pub fn main(_args: &[OsString]) -> Result<ExitCode, dfxvm::Error> {
    println!("Hello, world! (dfxvm mode)");
    Ok(ExitCode::SUCCESS)
}
