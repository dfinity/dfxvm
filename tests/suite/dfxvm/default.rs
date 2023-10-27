use crate::common::TempHomeDir;
use assert_cmd::prelude::*;

#[test]
fn default() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfxvm();
    cmd.arg("default");
    cmd.arg("0.5.8");

    cmd.assert().success().stdout("use dfx 0.5.8 by default\n");
}
