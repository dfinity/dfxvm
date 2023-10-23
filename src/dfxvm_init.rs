use crate::errors::dfxvm_init;
use std::ffi::OsString;
use std::process::ExitCode;

pub fn main(_args: &[OsString]) -> Result<ExitCode, dfxvm_init::Error> {
    println!("Hello, world! (dfxvm-init mode)");
    Ok(ExitCode::SUCCESS)
}
