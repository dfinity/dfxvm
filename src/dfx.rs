use crate::error::dfx;
use std::ffi::OsString;
use std::process::ExitCode;

pub fn main(_args: &[OsString]) -> Result<ExitCode, dfx::Error> {
    println!("Hello, world! (dfx mode)");
    Ok(ExitCode::SUCCESS)
}
