use crate::common::TempHomeDir;
use assert_cmd::prelude::*;
use predicates::str::*;

#[test]
fn normal() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script("0.3.2", "greetings from dfx 0.3.2");
    home_dir.create_executable_dfx_script("0.6.7", "it's dfx 0.6.7");

    home_dir
        .dfxvm()
        .arg("uninstall")
        .arg("0.3.2")
        .assert()
        .success()
        .stderr(contains("uninstalled dfx 0.3.2"));

    assert_eq!(home_dir.dfx_version_dirs(), vec!["0.6.7"]);
}

#[test]
fn version_not_installed() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script("0.5.7", "greetings from dfx 0.5.7");

    home_dir
        .dfxvm()
        .arg("uninstall")
        .arg("0.4.8")
        .assert()
        .success()
        .stderr(contains("dfx 0.4.8 is not installed"));

    assert_eq!(home_dir.dfx_version_dirs(), vec!["0.5.7"]);
}

#[test]
fn nothing_installed() {
    let home_dir = TempHomeDir::new();
    home_dir
        .dfxvm()
        .arg("uninstall")
        .arg("0.4.9")
        .assert()
        .success()
        .stderr(contains("dfx 0.4.9 is not installed"));

    assert!(!home_dir.versions_dir().exists());
}

#[test]
fn uninstall_dir_exists() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script("0.3.6", "greetings from dfx 0.3.6");
    home_dir.create_executable_dfx_script("0.7.2", "greetings from dfx 0.7.2");

    let uninstall_dir = home_dir.versions_dir().join(".uninstall-0.3.6");
    std::fs::create_dir(&uninstall_dir).unwrap();
    std::fs::write(uninstall_dir.join("any-filename"), "oh no").unwrap();

    home_dir
        .dfxvm()
        .arg("uninstall")
        .arg("0.3.6")
        .assert()
        .success()
        .stderr(contains("uninstalled dfx 0.3.6"));

    assert!(!home_dir.dfx_version_dir("0.3.6").exists());
    assert_eq!(home_dir.dfx_version_dirs(), vec!["0.7.2"]);
}

#[test]
fn uninstall_dir_exists_as_file() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script("0.3.6", "greetings from dfx 0.3.6");
    home_dir.create_executable_dfx_script("0.7.2", "greetings from dfx 0.7.2");

    let uninstall_dir = home_dir.versions_dir().join(".uninstall-0.3.6");
    std::fs::write(uninstall_dir, "oh no").unwrap();

    home_dir
        .dfxvm()
        .arg("uninstall")
        .arg("0.3.6")
        .assert()
        .success()
        .stderr(contains("uninstalled dfx 0.3.6"));

    assert!(!home_dir.dfx_version_dir("0.3.6").exists());
    assert_eq!(home_dir.dfx_version_dirs(), vec!["0.7.2"]);
}
