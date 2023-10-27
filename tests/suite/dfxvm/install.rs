use crate::common::TempHomeDir;
use assert_cmd::prelude::*;

#[test]
fn install() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfxvm();
    cmd.arg("install");
    cmd.arg("0.6.2");

    cmd.assert().success().stdout("install dfx 0.6.2\n");
}
