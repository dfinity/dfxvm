use crate::common::{project_dirs, ReleaseAsset, ReleaseServer, TempHomeDir};
use assert_cmd::prelude::*;
use predicates::str::*;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn successful_install() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.15.0", "echo 'this is dfx 0.15.0'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);

    let mut cmd = home_dir.dfxvm();
    cmd.arg("install").arg("0.15.0");
    cmd.assert().success();
    let dfx_path = home_dir.installed_dfx_path("0.15.0");
    assert!(dfx_path.exists());
    let mut dfx_cmd = Command::new(dfx_path);
    dfx_cmd.arg("--version");
    dfx_cmd.assert().success().stdout("this is dfx 0.15.0\n");
}

#[test]
fn successful_install_with_absolute_xdg_data_home() {
    let home_dir = TempHomeDir::new();
    let xdg_data_home = home_dir.path().join("xdg/data-home");
    let home_dir = home_dir.with_xdg_data_home(&xdg_data_home);
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.15.0", "echo 'this is dfx 0.15.0'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);

    let mut cmd = home_dir.dfxvm();
    cmd.arg("install").arg("0.15.0");
    cmd.assert().success();
    let dfx_path = home_dir.installed_dfx_path("0.15.0");
    assert!(dfx_path.exists());
    let mut dfx_cmd = Command::new(dfx_path);
    dfx_cmd.arg("--version");
    dfx_cmd.assert().success().stdout("this is dfx 0.15.0\n");
}

#[test]
fn successful_install_with_relative_xdg_data_home() {
    let home_dir = TempHomeDir::new().with_xdg_data_home(&PathBuf::from("relative/xdg/data-home"));
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.15.0", "echo 'this is dfx 0.15.0'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);

    let mut cmd = home_dir.dfxvm();
    cmd.arg("install").arg("0.15.0");
    cmd.assert().success();

    // relative xdg_data_home is same as no xdg_data_home
    let dfx_path = project_dirs::data_local_dir(home_dir.path(), None)
        .join("versions")
        .join("0.15.0")
        .join("dfx");
    assert!(dfx_path.exists());
    let mut dfx_cmd = Command::new(dfx_path);
    dfx_cmd.arg("--version");
    dfx_cmd.assert().success().stdout("this is dfx 0.15.0\n");
}

#[test]
fn successful_install_with_empty_xdg_data_home() {
    let home_dir = TempHomeDir::new().with_xdg_data_home(&PathBuf::from(""));
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.15.0", "echo 'this is dfx 0.15.0'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);

    let mut cmd = home_dir.dfxvm();
    cmd.arg("install").arg("0.15.0");
    cmd.assert().success();

    // empty xdg_data_home is same as no xdg_data_home
    let dfx_path = project_dirs::data_local_dir(home_dir.path(), None)
        .join("versions")
        .join("0.15.0")
        .join("dfx");
    assert!(dfx_path.exists());
    let mut dfx_cmd = Command::new(dfx_path);
    dfx_cmd.arg("--version");
    dfx_cmd.assert().success().stdout("this is dfx 0.15.0\n");
}

#[test]
fn incorrect_sha256() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.15.0", "echo 'this is dfx 0.15.0'");
    let wrong = ReleaseAsset::dfx_tarball("0.15.0", "echo 'not it'");
    let shasum = ReleaseAsset::sha256(&wrong);

    server.expect_get(&tarball);
    server.expect_get(&shasum);

    let version = "0.15.0";
    let mut cmd = home_dir.dfxvm();
    cmd.arg("install").arg(version);
    cmd.assert()
        .failure()
        .stderr(contains("checksum did not match"));
    assert!(!home_dir.dfx_version_dir(version).exists());
}

#[test]
fn version_does_not_exist() {
    let version = "0.13.6";

    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball(version, "echo 'this is dfx 0.13.6'");
    let sha256 = ReleaseAsset::sha256(&tarball);

    // install downloads the much smaller .sha256 first, so that if it's necessary
    // to retry, it doesn't have to re-download the larger file.
    server.expect_get_respond_not_found(&sha256);

    let mut cmd = home_dir.dfxvm();
    cmd.arg("install").arg(version);
    cmd.assert()
        .failure()
        .stderr(contains("no such version"))
        .stderr(is_match("404 Not Found.*.tar.gz.sha256").unwrap());
    assert!(!home_dir.dfx_version_dir(version).exists());
}
