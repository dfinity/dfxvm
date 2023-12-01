use crate::dfxvm_init::initialize::initialize;
use crate::dfxvm_init::ui::Confirmation;
use crate::error::dfxvm_init;
use clap::Parser;
use semver::Version;
use std::ffi::OsString;
use std::process::ExitCode;

/// The installer for dfxvm
#[derive(Parser)]
#[command(name = "dfxvm-init")]
pub struct Cli {
    /// The dfx version to install.  If not specified, installs the latest dfx version.
    #[clap(long)]
    dfx_version: Option<Version>,

    /// Automatically confirm options and proceed with install.
    #[clap(long)]
    proceed: bool,
}

pub async fn main(args: &[OsString]) -> Result<ExitCode, dfxvm_init::Error> {
    let opts = Cli::parse_from(args);

    let confirmation = if opts.proceed {
        Some(Confirmation::Proceed)
    } else {
        None
    };

    initialize(opts.dfx_version, confirmation).await?;

    Ok(ExitCode::SUCCESS)
}
