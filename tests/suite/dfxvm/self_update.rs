use crate::common::{ReleaseAsset, ReleaseServer, TempHomeDir};
use assert_cmd::prelude::*;
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


    let mut cmd = home_dir.installed_dfxvm();
    cmd.arg("self");
    cmd.arg("update");

    cmd.assert().success().stdout("update dfxvm to latest\n");
}
