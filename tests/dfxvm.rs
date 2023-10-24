use assert_cmd::prelude::*;
use common::dfxvm_command;

mod common;

#[test]
fn hello() {
    let mut cmd = dfxvm_command();

    cmd.assert().success().stdout("Hello, world!\n");
}
