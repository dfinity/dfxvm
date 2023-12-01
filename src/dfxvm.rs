mod cli;
mod default;
mod install;
mod list;
mod self_uninstall;
mod self_update;
mod uninstall;
mod update;

pub use cli::main;
pub use default::set_default;
pub use update::update;
