use crate::common::TempHomeDir;
use assert_cmd::prelude::*;
use predicates::str::contains;

#[test]
fn dispatch_to_dfx() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();

    cmd.assert().success().stdout("Hello, world! (dfx mode)\n");
}

#[test]
fn dispatch_to_dfxvm() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfxvm();

    cmd.assert()
        .success()
        .stdout("Hello, world! (dfxvm mode)\n");
}

#[test]
fn dispatch_to_dfxvm_init_exact() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfxvm_init();

    cmd.assert()
        .success()
        .stdout("Hello, world! (dfxvm-init mode)\n");
}

#[test]
fn dispatch_to_dfxvm_init_by_prefix() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.command("dfxvm-init (3)");

    cmd.assert()
        .success()
        .stdout("Hello, world! (dfxvm-init mode)\n");
}

#[test]
fn dispatch_to_unknown() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.command("called-something-else");

    cmd.assert()
        .failure()
        .stderr(contains(
            "error: Unrecognized executable name 'called-something-else'. Expect one of: dfx, dfxvm, dfxvm-init",
        ));
}
