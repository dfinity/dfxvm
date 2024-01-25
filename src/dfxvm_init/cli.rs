use crate::dfxvm::self_replace;
use crate::dfxvm_init::initialize::initialize;
use crate::dfxvm_init::plan::{
    DfxVersion::{Latest, Specific},
    PlanOptions,
};
use crate::dfxvm_init::ui::Confirmation;
use crate::error::dfxvm_init;
use crate::locations::Locations;
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
    yes: bool,

    /// Don't configure the PATH environment variable in profile scripts.
    #[clap(long)]
    no_modify_path: bool,
}

pub async fn main(args: &[OsString], locations: &Locations) -> Result<ExitCode, dfxvm_init::Error> {
    let arg1 = args.get(1).map(|a| &**a);
    if arg1 == Some("--self-replace".as_ref()) {
        self_replace(locations)?;
        return Ok(ExitCode::SUCCESS);
    }

    let opts = Cli::parse_from(args);

    let confirmation = if opts.yes {
        Some(Confirmation::Proceed)
    } else {
        None
    };

    let dfx_version = opts.dfx_version.map_or_else(|| Latest, Specific);

    let options = PlanOptions::new()
        .with_dfx_version(dfx_version)
        .with_modify_path(!opts.no_modify_path);

    initialize(options, confirmation, locations).await?;

    Ok(ExitCode::SUCCESS)
}
