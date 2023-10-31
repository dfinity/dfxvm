use std::env::args_os;
use std::ffi::OsString;
use std::process::ExitCode;

mod cli;
mod dfx;
mod dfxvm;
mod dfxvm_init;
mod errors;
#[macro_use]
mod log;

fn main() -> ExitCode {
    let args = args_os().collect::<Vec<OsString>>();
    cli::main(&args)
}
