use crate::common::file_contents::manifest_json;
use crate::common::{ReleaseServer, TempHomeDir};
use assert_cmd::prelude::*;
use predicates::str::*;
use std::fs::create_dir_all;

#[test]
fn no_versions_dir() {
    let home_dir = TempHomeDir::new();

    // looking for empty output, not an error
    home_dir.dfxvm().arg("list").assert().success().stdout("");
}

#[test]
fn empty_versions_dir() {
    let home_dir = TempHomeDir::new();

    create_dir_all(home_dir.versions_dir()).unwrap();

    home_dir.dfxvm().arg("list").assert().success().stdout("");
}

#[test]
fn single_version() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script("0.3.2", "...");

    home_dir
        .dfxvm()
        .arg("list")
        .assert()
        .success()
        .stdout("0.3.2\n");
}

#[test]
fn several_versions_no_default() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script("0.3.2", "...");
    home_dir.create_executable_dfx_script("0.14.2", "...");
    home_dir.create_executable_dfx_script("0.7.1", "...");
    home_dir.create_executable_dfx_script("0.15.0-beta.1", "...");

    home_dir
        .dfxvm()
        .arg("list")
        .assert()
        .success()
        .stdout("0.3.2\n0.7.1\n0.14.2\n0.15.0-beta.1\n");
}

#[test]
fn several_versions_with_default() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script("0.3.2", "...");
    home_dir.create_executable_dfx_script("0.14.2", "...");
    home_dir.create_executable_dfx_script("0.7.1", "...");
    home_dir.create_executable_dfx_script("0.15.0-beta.1", "...");
    home_dir.settings().write_default_version("0.7.1");

    home_dir
        .dfxvm()
        .arg("list")
        .assert()
        .success()
        .stdout("0.3.2\n0.7.1 (default)\n0.14.2\n0.15.0-beta.1\n");
}

#[test]
fn ignores_non_versions() {
    // directories whose names parse as a semver are installed versions.
    // everything else we ignore
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script("0.3.2", "...");
    home_dir.create_executable_dfx_script("0.14.2", "...");
    home_dir.create_executable_dfx_script("0.7.1", "...");
    home_dir.create_executable_dfx_script("0.15.0-beta.1", "...");
    home_dir.create_executable_dfx_script(".uninstall-0.15.0", "...");
    home_dir.create_executable_dfx_script(".install-tempXYZ", "...");
    std::fs::write(home_dir.versions_dir().join("0.1.1"), "...").unwrap();
    std::fs::write(home_dir.versions_dir().join(".DS_Store"), "...").unwrap();
    std::fs::write(home_dir.versions_dir().join("arbitrary"), "...").unwrap();

    home_dir
        .dfxvm()
        .arg("list")
        .assert()
        .success()
        .stdout("0.3.2\n0.7.1\n0.14.2\n0.15.0-beta.1\n");
}

#[test]
fn remote_versions() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_get_manifest(&manifest_json("0.14.6"));

    home_dir
        .dfxvm()
        .arg("list")
        .arg("--available")
        .assert()
        .success()
        .stderr(is_match("info: fetching http://.*/manifest.json").unwrap())
        .stdout(contains("0.5.2").count(1))
        .stdout(contains("0.5.0").count(1));
}

#[test]
fn remote_versions_with_limit() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_get_manifest(&manifest_json("0.14.6"));

    home_dir
        .dfxvm()
        .arg("list")
        .arg("--available")
        .arg("--limit")
        .arg("1")
        .assert()
        .success()
        .stderr(is_match("info: fetching http://.*/manifest.json").unwrap())
        .stdout(contains("0.5.2").count(1))
        .stdout(contains("0.5.0").count(0));
}
