mod executable;
pub mod file_contents;
pub mod paths;
pub mod project_dirs;
mod release_asset;
mod release_server;
mod settings;
mod temp_home_dir;

pub use release_asset::ReleaseAsset;
pub use release_server::ReleaseServer;
pub use settings::Settings;
pub use temp_home_dir::TempHomeDir;

pub fn dfxvm_path() -> &'static str {
    env!("CARGO_BIN_EXE_dfxvm")
}
