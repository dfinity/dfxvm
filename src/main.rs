use std::env::args_os;
use std::ffi::OsString;
use std::process::ExitCode;

#[macro_use]
mod log;

mod cli;
mod dfx;
mod dfxvm;
mod dfxvm_init;
mod errors;

fn main() -> ExitCode {
    let args = args_os().collect::<Vec<OsString>>();
    cli::main(&args)
}
