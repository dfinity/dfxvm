use crate::common::file_contents::manifest_json;
use crate::common::{ReleaseAsset, ReleaseServer, TempHomeDir};
use assert_cmd::prelude::*;
use predicates::str::*;
use std::process::Command;

#[test]
fn new_version_available() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    home_dir.settings().write_default_version("0.7.2");

    let tarball = ReleaseAsset::dfx_tarball("0.14.6", "echo 'this is dfx 0.14.6'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);

    server.expect_get_manifest(&manifest_json("0.14.6"));

    home_dir
        .dfxvm()
        .arg("update")
        .assert()
        .success()
        .stderr(is_match("info: fetching http://.*/manifest.json").unwrap())
        .stderr(contains("info: latest dfx version is 0.14.6"))
        .stderr(contains("info: installed dfx 0.14.6"))
        .stderr(contains("info: set default version to dfx 0.14.6"));

    assert_eq!(home_dir.settings().read_default_version(), "0.14.6");

    // call through dfxvm (as dfx)
    home_dir
        .dfx()
        .assert()
        .success()
        .stdout("this is dfx 0.14.6\n");

    // call directly
    Command::new(home_dir.installed_dfx_path("0.14.6"))
        .assert()
        .success()
        .stdout("this is dfx 0.14.6\n");
}

#[test]
fn different_but_older_version_available() {
    // update updates to the latest tag, even if it's an "older" version that what is
    // the current default.
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    home_dir.settings().write_default_version("0.15.2");

    let tarball = ReleaseAsset::dfx_tarball("0.15.1", "echo 'this is dfx 0.15.1'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);

    server.expect_get_manifest(&manifest_json("0.15.1"));

    home_dir
        .dfxvm()
        .arg("update")
        .assert()
        .success()
        .stderr(is_match("info: fetching http://.*/manifest.json").unwrap())
        .stderr(contains("info: latest dfx version is 0.15.1"))
        .stderr(contains("info: installed dfx 0.15.1"))
        .stderr(contains("info: set default version to dfx 0.15.1"));

    assert_eq!(home_dir.settings().read_default_version(), "0.15.1");

    // call through dfxvm (as dfx)
    home_dir
        .dfx()
        .assert()
        .success()
        .stdout("this is dfx 0.15.1\n");
}

#[test]
fn no_new_version_available() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    home_dir.settings().write_default_version("0.14.8");
    home_dir.create_executable_dfx_script("0.14.8", "echo this is the installed 0.14.8 dfx");

    server.expect_get_manifest(&manifest_json("0.14.8"));

    home_dir
        .dfxvm()
        .arg("update")
        .assert()
        .success()
        .stderr(is_match("info: fetching http://.*/manifest.json").unwrap())
        .stderr(contains("info: latest dfx version is 0.14.8"))
        .stderr(contains("info: using existing install for dfx 0.14.8"))
        .stderr(contains("info: dfx 0.14.8 is already the default version"));

    assert_eq!(home_dir.settings().read_default_version(), "0.14.8");

    home_dir
        .dfx()
        .assert()
        .success()
        .stdout("this is the installed 0.14.8 dfx\n");
}

#[test]
fn new_version_already_installed() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    home_dir.create_executable_dfx_script("0.14.8", "echo this is the installed 0.14.8 dfx");
    home_dir.create_executable_dfx_script("0.14.9", "echo this is the installed 0.14.9 dfx");

    home_dir.settings().write_default_version("0.14.8");

    home_dir
        .dfx()
        .assert()
        .success()
        .stdout("this is the installed 0.14.8 dfx\n");

    server.expect_get_manifest(&manifest_json("0.14.9"));

    home_dir
        .dfxvm()
        .arg("update")
        .assert()
        .success()
        .stderr(is_match("info: fetching http://.*/manifest.json").unwrap())
        .stderr(contains("info: latest dfx version is 0.14.9"))
        .stderr(contains("info: using existing install for dfx 0.14.9"))
        .stderr(contains("info: set default version to dfx 0.14.9"));

    assert_eq!(home_dir.settings().read_default_version(), "0.14.9");

    home_dir
        .dfx()
        .assert()
        .success()
        .stdout("this is the installed 0.14.9 dfx\n");
}

#[test]
fn update_from_no_settings() {
    // actually can't test this without a setting file, because the test URLs live there.
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.14.9", "echo 'this is dfx 0.14.9'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);

    server.expect_get_manifest(&manifest_json("0.14.9"));

    home_dir
        .dfxvm()
        .arg("update")
        .assert()
        .success()
        .stderr(is_match("info: fetching http://.*/manifest.json").unwrap())
        .stderr(contains("info: latest dfx version is 0.14.9"))
        .stderr(contains("info: installing dfx 0.14.9"))
        .stderr(contains("info: set default version to dfx 0.14.9"));

    assert_eq!(home_dir.settings().read_default_version(), "0.14.9");

    home_dir
        .dfx()
        .assert()
        .success()
        .stdout("this is dfx 0.14.9\n");
}
