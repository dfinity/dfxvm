use std::process::Command;

fn dfxvm_path() -> &'static str {
    env!("CARGO_BIN_EXE_dfxvm")
}

pub fn dfxvm_command() -> Command {
    Command::new(dfxvm_path())
}
