use crate::common::{ReleaseAsset, ReleaseServer, TempHomeDir};
use assert_cmd::prelude::*;
use predicates::str::contains;

#[test]
fn change_default() {
    // This is the normal case:
    //   - the version is set to something (by dfxvm-init, dfxvm update, or dfxvm default)
    //   - you are changing it to something else
    //  You'd have to edit the config file to get something else.
    let home_dir = TempHomeDir::new();

    home_dir.create_executable_dfx_script("0.5.6", "echo 'hi from 0.5.6'");
    home_dir.create_executable_dfx_script("0.6.3", "echo 'hi from 0.6.3'");
    home_dir.settings().write_default_version("0.5.6");

    home_dir.dfx().assert().success().stdout("hi from 0.5.6\n");

    home_dir
        .dfxvm()
        .arg("default")
        .arg("0.6.3")
        .assert()
        .success()
        .stderr(contains("using existing install for dfx 0.6.3"))
        .stderr(contains("set default version to dfx 0.6.3"));

    assert_eq!(home_dir.settings().read_default_version(), "0.6.3");

    home_dir.dfx().assert().success().stdout("hi from 0.6.3\n");
}

#[test]
fn display_default() {
    let home_dir = TempHomeDir::new();
    home_dir.settings().write_default_version("0.2.3-beta.6");

    home_dir
        .dfxvm()
        .arg("default")
        .assert()
        .success()
        .stdout("0.2.3-beta.6\n");
}

#[test]
fn display_default_no_config_file() {
    let home_dir = TempHomeDir::new();

    home_dir
        .dfxvm()
        .arg("default")
        .assert()
        .failure()
        .stderr("error: no default dfx version configured\n");
}

#[test]
fn display_default_no_default_version() {
    let home_dir = TempHomeDir::new();
    home_dir.settings().write("{}");

    // what we are looking for: a nice error message that is not about a missing json field
    home_dir
        .dfxvm()
        .arg("default")
        .assert()
        .failure()
        .stderr("error: no default dfx version configured\n");
}

#[test]
fn set_default_same_version() {
    let home_dir = TempHomeDir::new();

    home_dir.create_executable_dfx_script("0.5.6", "echo 'hi from 0.5.6'");
    home_dir.create_executable_dfx_script("0.6.3", "echo 'hi from 0.6.3'");
    home_dir.settings().write_default_version("0.5.6");

    home_dir.dfx().assert().success().stdout("hi from 0.5.6\n");

    home_dir
        .dfxvm()
        .arg("default")
        .arg("0.5.6")
        .assert()
        .success()
        .stderr(contains("using existing install for dfx 0.5.6"))
        .stderr(contains("dfx 0.5.6 is already the default version"));

    assert_eq!(home_dir.settings().read_default_version(), "0.5.6");

    home_dir.dfx().assert().success().stdout("hi from 0.5.6\n");
}

#[test]
fn set_default_no_config_file() {
    let home_dir = TempHomeDir::new();

    home_dir.create_executable_dfx_script("0.5.8", "echo 'hi from 0.5.8'");

    home_dir
        .dfx()
        .assert()
        .failure()
        .stderr(contains("Unable to determine which dfx version to call"));

    home_dir
        .dfxvm()
        .arg("default")
        .arg("0.5.8")
        .assert()
        .success()
        .stderr(contains("using existing install for dfx 0.5.8"))
        .stderr(contains("set default version to dfx 0.5.8"));

    assert_eq!(home_dir.settings().read_default_version(), "0.5.8");

    home_dir.dfx().assert().success().stdout("hi from 0.5.8\n");
}

#[test]
fn set_default_no_default_version() {
    let home_dir = TempHomeDir::new();

    home_dir.create_executable_dfx_script("0.5.8", "echo 'hi from 0.5.8'");
    home_dir.settings().write("{}");

    home_dir
        .dfx()
        .assert()
        .failure()
        .stderr(contains("Unable to determine which dfx version to call"));

    home_dir
        .dfxvm()
        .arg("default")
        .arg("0.5.8")
        .assert()
        .success()
        .stderr(contains("using existing install for dfx 0.5.8"))
        .stderr(contains("set default version to dfx 0.5.8"));

    assert_eq!(home_dir.settings().read_default_version(), "0.5.8");

    // doesn't write default values for other fields
    assert_eq!(home_dir.settings().sorted_keys(), ["default_version"]);

    home_dir.dfx().assert().success().stdout("hi from 0.5.8\n");
}

#[test]
fn installs_if_not_installed() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    home_dir.create_executable_dfx_script("0.5.6", "echo 'hi from 0.5.6'");
    home_dir.settings().write_default_version("0.5.6");

    let tarball = ReleaseAsset::dfx_tarball("0.2.7", "echo this is dfx 0.2.7");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);

    home_dir
        .dfxvm()
        .arg("default")
        .arg("0.2.7")
        .assert()
        .success()
        .stderr(contains("installing dfx 0.2.7"))
        .stderr(contains("set default version to dfx 0.2.7"));

    assert_eq!(home_dir.settings().read_default_version(), "0.2.7");

    home_dir
        .dfx()
        .assert()
        .success()
        .stdout("this is dfx 0.2.7\n");
}

#[test]
fn version_not_found() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    home_dir.create_executable_dfx_script("0.5.6", "echo 'hi from 0.5.6'");
    home_dir.settings().write_default_version("0.5.6");

    let tarball = ReleaseAsset::dfx_tarball("0.2.3", "echo this is dfx 0.2.3");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get_respond_not_found(&sha256);

    home_dir
        .dfxvm()
        .arg("default")
        .arg("0.2.3")
        .assert()
        .failure()
        .stderr(contains("installing dfx 0.2.3"))
        .stderr(contains("no such version"));

    assert_eq!(home_dir.settings().read_default_version(), "0.5.6");

    home_dir.dfx().assert().success().stdout("hi from 0.5.6\n");
}

#[test]
fn default_does_not_write_other_field_defaults() {
    // looking for here: does not write default values of download_url_template or manifest_url
    let home_dir = TempHomeDir::new();

    home_dir.create_executable_dfx_script("0.5.6", "echo 'hi from 0.5.6'");
    home_dir.create_executable_dfx_script("0.6.3", "echo 'hi from 0.6.3'");
    home_dir.settings().write_default_version("0.5.6");

    home_dir.dfx().assert().success().stdout("hi from 0.5.6\n");

    home_dir
        .dfxvm()
        .arg("default")
        .arg("0.6.3")
        .assert()
        .success()
        .stderr(contains("using existing install for dfx 0.6.3"))
        .stderr(contains("set default version to dfx 0.6.3"));

    assert_eq!(home_dir.settings().read_default_version(), "0.6.3");
    assert_eq!(home_dir.settings().sorted_keys(), ["default_version"]);

    home_dir.dfx().assert().success().stdout("hi from 0.6.3\n");
}

#[test]
fn default_leaves_unknown_settings() {
    let home_dir = TempHomeDir::new();

    home_dir.create_executable_dfx_script("0.5.6", "echo 'hi from 0.5.6'");
    home_dir.create_executable_dfx_script("0.6.3", "echo 'hi from 0.6.3'");
    home_dir
        .settings()
        .write(r#"{ "default_version": "0.5.6", "something_else": "whatever" }"#);

    home_dir.dfx().assert().success().stdout("hi from 0.5.6\n");

    home_dir
        .dfxvm()
        .arg("default")
        .arg("0.6.3")
        .assert()
        .success()
        .stderr(contains("using existing install for dfx 0.6.3"))
        .stderr(contains("set default version to dfx 0.6.3"));

    assert_eq!(home_dir.settings().read_default_version(), "0.6.3");
    assert_eq!(
        home_dir.settings().sorted_keys(),
        ["default_version", "something_else"]
    );

    home_dir.dfx().assert().success().stdout("hi from 0.6.3\n");
}
