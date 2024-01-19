use crate::common::file_contents::dist_manifest_json;
use crate::common::{ReleaseAsset, ReleaseServer, TempHomeDir};
use assert_cmd::prelude::*;
use predicates::str::contains;
use semver::Version;

fn different_version(patch_diff: i64) -> String {
    let current_version = env!("CARGO_PKG_VERSION");
    let ver = Version::parse(current_version).unwrap();
    let patch = (ver.patch as i64 + patch_diff) as u64;
    format!("{}.{}.{}", ver.major, ver.minor, patch)
}

fn older_version() -> String {
    different_version(-1)
}

fn newer_version() -> String {
    different_version(1)
}

#[test]
fn self_update_older() {
    self_update(&older_version());
}

#[test]
fn self_update_newer() {
    self_update(&newer_version());
}

#[test]
fn dfx_cleans_up_after_self_update() {
    let home_dir = self_update(&newer_version());
    assert_eq!(
        home_dir.installed_binaries(),
        ["dfx", "dfxvm", "dfxvm-init-self-update"]
    );

    home_dir.dfx().arg("anything").assert().failure();
    assert_eq!(home_dir.installed_binaries(), ["dfx", "dfxvm"]);
}

#[test]
fn dfxvm_cleans_up_after_self_update() {
    let home_dir = self_update(&newer_version());

    assert_eq!(
        home_dir.installed_binaries(),
        ["dfx", "dfxvm", "dfxvm-init-self-update"]
    );

    home_dir
        .installed_dfxvm()
        .arg("anything")
        .assert()
        .failure();
    assert_eq!(home_dir.installed_binaries(), ["dfx", "dfxvm"]);
}

fn self_update(to_version: &str) -> TempHomeDir {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::altered_dfxvm_tarball();
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);
    server.expect_get_dist_manifest(&dist_manifest_json(to_version));

    // before we do this, the installed dfxvm and dfx proxy should be the one we're testing with
    home_dir.install_dfxvm_bin();
    home_dir.install_dfxvm_bin_as_dfx_proxy();
    let installed_dfxvm = std::fs::read(home_dir.installed_dfxvm_path()).unwrap();
    let built_dfxvm = std::fs::read(crate::common::dfxvm_path()).unwrap();
    let altered_dfxvm = ReleaseAsset::altered_dfxvm_binary();
    assert!(
        installed_dfxvm == built_dfxvm,
        "installed dfxvm is not the built dfxvm"
    );
    assert!(
        installed_dfxvm != altered_dfxvm,
        "installed dfxvm is the altered dfxvm"
    );

    let mut cmd = home_dir.installed_dfxvm();
    cmd.arg("self");
    cmd.arg("update");

    cmd.assert()
        .success()
        .stderr(contains("checking for self-update"))
        .stderr(contains("verified checksum"));

    // // after self update, the installed dfxvm should be the one we downloaded
    let installed_dfxvm = std::fs::read(home_dir.installed_dfxvm_path()).unwrap();
    assert!(
        installed_dfxvm != built_dfxvm,
        "installed dfxvm is still the built dfxvm"
    );
    assert!(
        installed_dfxvm == altered_dfxvm,
        "installed dfxvm is not the altered dfxvm"
    );
    // as should the dfx proxy binary
    let installed_dfx_proxy = std::fs::read(home_dir.installed_dfx_proxy_path()).unwrap();
    assert!(
        installed_dfx_proxy != built_dfxvm,
        "installed dfx proxy is still the built dfxvm"
    );
    assert!(
        installed_dfx_proxy == altered_dfxvm,
        "installed dfx proxy is not the altered dfxvm"
    );

    assert_eq!(
        home_dir.installed_binaries(),
        ["dfx", "dfxvm", "dfxvm-init-self-update"]
    );

    home_dir
}

#[test]
fn unchanged() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let current_version = env!("CARGO_PKG_VERSION");

    server.expect_get_dist_manifest(&dist_manifest_json(current_version));

    // before we do this, the installed dfxvm and dfx proxy should be the one we're testing with
    home_dir.install_dfxvm_bin();
    home_dir.install_dfxvm_bin_as_dfx_proxy();

    let mut cmd = home_dir.installed_dfxvm();
    cmd.arg("self");
    cmd.arg("update");

    cmd.assert()
        .success()
        .stderr(contains("checking for self-update"))
        .stderr(contains(format!("dfxvm unchanged - {current_version}")));
}

#[test]
fn incorrect_sha256() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::altered_dfxvm_tarball();
    let wrong = ReleaseAsset {
        contents: b"not the right contents".to_vec(),
        ..tarball.clone()
    };
    let sha256 = ReleaseAsset::sha256(&wrong);
    server.expect_get(&tarball);
    server.expect_get(&sha256);
    server.expect_get_dist_manifest(&dist_manifest_json(&newer_version()));

    // before we do this, the installed dfxvm and dfx proxy should be the one we're testing with
    home_dir.install_dfxvm_bin();
    home_dir.install_dfxvm_bin_as_dfx_proxy();

    let mut cmd = home_dir.installed_dfxvm();
    cmd.arg("self");
    cmd.arg("update");

    cmd.assert()
        .failure()
        .stderr(contains("checking for self-update"))
        .stderr(contains("checksum did not match"));
}
