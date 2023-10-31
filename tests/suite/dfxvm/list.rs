use crate::common::TempHomeDir;
use assert_cmd::prelude::*;

#[test]
fn list() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfxvm();
    cmd.arg("list");

    cmd.assert()
        .success()
        .stdout("list installed dfx versions\n");
}
