use crate::common::TempHomeDir;
use assert_cmd::prelude::*;

#[test]
fn self_update() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfxvm();
    cmd.arg("self");
    cmd.arg("update");

    cmd.assert().success().stdout("update dfxvm to latest\n");
}
