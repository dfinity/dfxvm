use std::env::args_os;
use std::ffi::OsString;
use std::process::ExitCode;

#[macro_use]
mod log;

mod cli;
mod dfx;
mod dfxvm;
mod dfxvm_init;
mod dist_manifest;
mod download;
mod env;
mod error;
mod fs;
mod installation;
mod json;
mod locations;
mod settings;
mod style;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let args = args_os().collect::<Vec<OsString>>();
    cli::main(&args).await
}
