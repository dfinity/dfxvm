mod temp_home_dir;

pub use temp_home_dir::TempHomeDir;

fn dfxvm_path() -> &'static str {
    env!("CARGO_BIN_EXE_dfxvm")
}
