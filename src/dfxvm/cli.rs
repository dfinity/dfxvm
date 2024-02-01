use crate::dfxvm::{
    cleanup_self_updater, default::default, install::install, list::list,
    self_uninstall::self_uninstall, self_update::self_update, uninstall::uninstall, update::update,
};
use crate::error::dfxvm;
use crate::locations::Locations;
use clap::{Parser, Subcommand};
use semver::Version;
use std::ffi::OsString;
use std::process::ExitCode;

/// The dfx version manager
#[derive(Parser)]
#[command(name = "dfxvm", arg_required_else_help = true, version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Default(DefaultOpts),
    Install(InstallOpts),
    List(ListOpts),
    #[command(name = "self")]
    SelfCmd(SelfOpts),
    Uninstall(UninstallOpts),
    Update(UpdateOpts),
}

/// Install a version of dfx
#[derive(Parser)]
pub struct InstallOpts {
    /// dfx version to install
    version: Version,
}

/// Set a dfx version to be the default, installing if necessary
#[derive(Parser)]
pub struct DefaultOpts {
    /// dfx version to use by default
    version: Option<Version>,
}

/// List installed versions of dfx
#[derive(Parser)]
pub struct ListOpts {}

/// Uninstall a version of dfx
#[derive(Parser)]
pub struct UninstallOpts {
    /// dfx version to uninstall
    version: Version,
}

/// Update to latest version of dfx
#[derive(Parser)]
pub struct UpdateOpts {}

/// Manage dfxvm itself
#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub struct SelfOpts {
    #[command(subcommand)]
    command: SelfCommand,
}

#[derive(Subcommand)]
pub enum SelfCommand {
    Update(SelfUpdateOpts),
    Uninstall(SelfUninstallOpts),
}

/// Update to the latest version of dfxvm
#[derive(Parser)]
pub struct SelfUpdateOpts {}

/// Uninstall dfxvm and all versions of dfx
#[derive(Parser)]
pub struct SelfUninstallOpts {
    /// Automatically confirm un-installation.
    #[clap(long)]
    yes: bool,
}

pub async fn main(args: &[OsString], locations: &Locations) -> Result<ExitCode, dfxvm::Error> {
    cleanup_self_updater(locations)?;
    let cli = Cli::parse_from(args);
    match cli.command {
        Command::Default(opts) => default(opts.version, locations).await?,
        Command::Install(opts) => install(opts.version, locations).await?,
        Command::List(_opts) => list(locations)?,
        Command::SelfCmd(opts) => match opts.command {
            SelfCommand::Update(_opts) => self_update(locations).await?,
            SelfCommand::Uninstall(opts) => self_uninstall(opts.yes, locations)?,
        },
        Command::Uninstall(opts) => uninstall(opts.version, locations)?,
        Command::Update(_opts) => update(locations).await?,
    };
    Ok(ExitCode::SUCCESS)
}
