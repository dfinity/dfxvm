use crate::common::TempHomeDir;
use assert_cmd::prelude::*;
use predicates::str::contains;

#[test]
fn dispatch_to_dfx() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();

    home_dir.create_executable_dfx_script("4.6.0", "echo 'Hello, world! (dfx 0.4.6 script)'");

    cmd.arg("+4.6.0");
    cmd.assert()
        .success()
        .stdout("Hello, world! (dfx 0.4.6 script)\n");
}

#[test]
fn dispatch_to_dfxvm() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfxvm();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(contains("Usage: dfxvm <COMMAND>"));
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
            "error: unrecognized executable name 'called-something-else'; expect one of: dfx, dfxvm, dfxvm-init",
        ));
}
