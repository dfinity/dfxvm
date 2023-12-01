use crate::common::{file_contents::manifest_json, ReleaseAsset, ReleaseServer, TempHomeDir};
use assert_cmd::prelude::*;
use predicates::str::*;
use std::os::unix::fs::PermissionsExt;

#[test]
fn default_installation() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.15.0", "echo 'this is dfx 0.15.0'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);
    server.expect_get_manifest(&manifest_json("0.15.0"));

    #[cfg(target_os = "macos")]
    let expected_env_path = "$HOME/Library/Application Support/org.dfinity.dfx/env";
    #[cfg(target_os = "linux")]
    let expected_env_path = "$HOME/.local/share/dfx/env";

    home_dir
        .dfxvm_init()
        .arg("--proceed")
        .assert()
        .success()
        .stdout(contains("dfxvm is installed now."))
        .stdout(contains("To configure your shell, run:"))
        .stdout(contains(format!(r#"  source "{expected_env_path}""#)));

    assert!(home_dir.installed_dfx_path("0.15.0").exists());
    assert_eq!(home_dir.settings().read_default_version(), "0.15.0");

    home_dir
        .bash_script_command("dfx --version")
        .assert()
        .failure();
    home_dir
        .bash_script_command(&format!(
            r#"
        source "{expected_env_path}"
        dfx --version"#
        ))
        .assert()
        .success()
        .stdout("this is dfx 0.15.0\n");

    home_dir
        .bash_script_command("dfxvm default")
        .assert()
        .failure();
    home_dir
        .bash_script_command(&format!(
            r#"
        source "{expected_env_path}"
        dfxvm default"#
        ))
        .assert()
        .success()
        .stdout("0.15.0\n");
}

#[test]
fn specific_dfx_version() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.14.7", "echo 'this is dfx 0.14.7'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);

    #[cfg(target_os = "macos")]
    let expected_env_path = "$HOME/Library/Application Support/org.dfinity.dfx/env";
    #[cfg(target_os = "linux")]
    let expected_env_path = "$HOME/.local/share/dfx/env";

    home_dir
        .dfxvm_init()
        .arg("--dfx-version")
        .arg("0.14.7")
        .arg("--proceed")
        .assert()
        .success()
        .stdout(contains("dfxvm is installed now."))
        .stdout(contains("To configure your shell, run:"))
        .stdout(contains(format!(r#"  source "{expected_env_path}""#)));

    assert!(home_dir.installed_dfx_path("0.14.7").exists());
    assert_eq!(home_dir.settings().read_default_version(), "0.14.7");

    home_dir
        .bash_script_command("dfx --version")
        .assert()
        .failure();
    home_dir
        .bash_script_command(&format!(
            r#"
        source "{expected_env_path}"
        dfx --version"#
        ))
        .assert()
        .success()
        .stdout("this is dfx 0.14.7\n");

    home_dir
        .bash_script_command("dfxvm default")
        .assert()
        .failure();
    home_dir
        .bash_script_command(&format!(
            r#"
        source "{expected_env_path}"
        dfxvm default"#
        ))
        .assert()
        .success()
        .stdout("0.14.7\n");
}

#[test]
fn xdg_data_home_set() {
    let home_dir = TempHomeDir::new();
    let xdg_data_home = home_dir.path().join(".custom/xdg/data-home");
    let home_dir = home_dir.with_xdg_data_home(&xdg_data_home);
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.14.7", "echo 'this is dfx 0.14.7'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);

    #[cfg(target_os = "macos")]
    let expected_env_path = "$HOME/Library/Application Support/org.dfinity.dfx/env";
    #[cfg(target_os = "linux")]
    let expected_env_path = "$XDG_DATA_HOME/dfx/env";

    home_dir
        .dfxvm_init()
        .arg("--dfx-version")
        .arg("0.14.7")
        .arg("--proceed")
        .assert()
        .success()
        .stdout(contains("dfxvm is installed now."))
        .stdout(contains("To configure your shell, run:"))
        .stdout(contains(format!(r#"  source "{expected_env_path}""#)));

    assert!(home_dir.installed_dfx_path("0.14.7").exists());
    assert_eq!(home_dir.settings().read_default_version(), "0.14.7");

    home_dir
        .bash_script_command("dfx --version")
        .assert()
        .failure();
    home_dir
        .bash_script_command(&format!(
            r#"
        source "{expected_env_path}"
        dfx --version"#
        ))
        .assert()
        .success()
        .stdout("this is dfx 0.14.7\n");

    home_dir
        .bash_script_command("dfxvm default")
        .assert()
        .failure();
    home_dir
        .bash_script_command(&format!(
            r#"
        source "{expected_env_path}"
        dfxvm default"#
        ))
        .assert()
        .success()
        .stdout("0.14.7\n");
}

#[test]
fn adds_permissions() {
    // has to at least be r-x------ in order to execute and copy
    sets_permissions_from(0o500);
}

#[test]
fn removes_permissions() {
    sets_permissions_from(0o777);
}

fn sets_permissions_from(from_mode: u32) {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.11.8", "echo 'this is dfx 0.11.8'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);
    server.expect_get_manifest(&manifest_json("0.11.8"));

    let dfxvm_init = home_dir.dfxvm_as_file_named("dfxvm-init");
    std::fs::set_permissions(&dfxvm_init, std::fs::Permissions::from_mode(from_mode)).unwrap();
    let metadata = std::fs::metadata(&dfxvm_init).unwrap();
    const MODE_FILE: u32 = 0o100000;
    assert_eq!(metadata.permissions().mode(), MODE_FILE | from_mode);

    home_dir.dfxvm_init().arg("--proceed").assert().success();

    // should have left the source file alone
    let metadata = std::fs::metadata(&dfxvm_init).unwrap();
    assert_eq!(metadata.permissions().mode(), MODE_FILE | from_mode);

    // the installed dfxvm should have the correct permissions
    let metadata = std::fs::metadata(home_dir.installed_dfxvm_path()).unwrap();
    assert_eq!(metadata.permissions().mode(), MODE_FILE | 0o755);

    // and the installed dfx proxy
    let metadata = std::fs::metadata(home_dir.installed_dfx_proxy_path()).unwrap();
    assert_eq!(metadata.permissions().mode(), MODE_FILE | 0o755);
}
