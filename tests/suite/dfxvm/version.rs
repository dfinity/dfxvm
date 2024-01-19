use crate::common::TempHomeDir;
use assert_cmd::prelude::*;

#[test]
fn display_default_no_config_file() {
    let home_dir = TempHomeDir::new();

    home_dir
        .dfxvm()
        .arg("--version")
        .assert()
        .success()
        .stdout(format!("dfxvm {}\n", env!("CARGO_PKG_VERSION")));
}
