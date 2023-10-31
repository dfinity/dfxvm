use crate::common::TempHomeDir;
use assert_cmd::prelude::*;

#[test]
fn uninstall() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfxvm();
    cmd.arg("uninstall");
    cmd.arg("0.4.8");

    cmd.assert().success().stdout("uninstall dfx 0.4.8\n");
}
