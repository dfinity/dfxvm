use std::env::args_os;
use std::ffi::OsString;
use std::process::ExitCode;

mod cli;
mod dfx;
mod dfxvm;
mod dfxvm_init;
mod errors;

fn main() -> anyhow::Result<ExitCode> {
    let args = args_os().collect::<Vec<OsString>>();
    let exit_code = cli::dispatch(&args)?;
    Ok(exit_code)
}
