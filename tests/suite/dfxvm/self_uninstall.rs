use crate::common::TempHomeDir;
use assert_cmd::prelude::*;

#[test]
fn self_uninstall() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfxvm();
    cmd.arg("self");
    cmd.arg("uninstall");

    cmd.assert()
        .success()
        .stdout("uninstall dfxvm and all dfx versions\n");
}
