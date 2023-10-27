use crate::common::TempHomeDir;
use assert_cmd::prelude::*;

#[test]
fn update() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfxvm();
    cmd.arg("update");

    cmd.assert().success().stdout("update to latest dfx\n");
}
