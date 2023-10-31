use crate::dfxvm::{
    default::default, install::install, list::list, self_uninstall::self_uninstall,
    self_update::self_update, uninstall::uninstall, update::update,
};
use crate::errors::dfxvm;
use clap::{Parser, Subcommand};
use semver::Version;
use std::ffi::OsString;
use std::process::ExitCode;

/// The dfx version manager
#[derive(Parser)]
#[command(name = "dfxvm", arg_required_else_help = true)]
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
    SelfCommand(SelfOpts),
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
    version: Version,
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
pub struct SelfUninstallOpts {}

pub fn main(args: &[OsString]) -> Result<ExitCode, dfxvm::Error> {
    let cli = Cli::parse_from(args);
    let exit_code = match cli.command {
        Command::Default(opts) => default(opts.version)?,
        Command::Install(opts) => install(opts.version)?,
        Command::List(_opts) => list()?,
        Command::SelfCommand(opts) => match opts.command {
            SelfCommand::Update(_opts) => self_update()?,
            SelfCommand::Uninstall(_opts) => self_uninstall()?,
        },
        Command::Uninstall(opts) => uninstall(opts.version)?,
        Command::Update(_opts) => update()?,
    };
    Ok(exit_code)
}
