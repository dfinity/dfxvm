use crate::common::{ReleaseAsset, ReleaseServer, TempHomeDir};
use assert_cmd::prelude::*;
use predicates::str::contains;
use semver::Version;
use crate::common::file_contents::{bash_script, dfx_tar_gz, dist_manifest_json, manifest_json};

// tricky about testing this:
// - we need to test that the dfxvm binary is updated
// - we only have the current dfxvm binary to test with
// - we'll try copying the binary and appending a few bytes to it.

#[test]
fn self_update() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);


    let tarball = ReleaseAsset::altered_dfxvm_tarball("0.1.2");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);
    server.expect_get_dist_manifest(&dist_manifest_json("0.1.3"));

    // before we do this, the installed dfxvm should be the one we're testing with
    home_dir.install_dfxvm_bin();
    let installed_dfxvm = std::fs::read(home_dir.installed_dfxvm_path()).unwrap();
    let built_dfxvm = std::fs::read(crate::common::dfxvm_path()).unwrap();
    let altered_dfxvm = ReleaseAsset::altered_dfxvm_binary();
    assert!(installed_dfxvm == built_dfxvm, "installed dfxvm is not the built dfxvm");
    assert!(installed_dfxvm != altered_dfxvm, "installed dfxvm is the altered dfxvm");

    let mut cmd = home_dir.installed_dfxvm();
    cmd.arg("self");
    cmd.arg("update");
    //cmd.assert().failure();
    cmd.assert().success()
        .stdout(contains("update dfxvm to latest\n"))
        .stderr(contains("verified checksum"));

    // // after self update, the installed dfxvm should be the one we downloaded
    let installed_dfxvm = std::fs::read(home_dir.installed_dfxvm_path()).unwrap();
    assert!(installed_dfxvm != built_dfxvm, "installed dfxvm is still the built dfxvm");
    assert!(installed_dfxvm == altered_dfxvm, "installed dfxvm is not the altered dfxvm");
}
