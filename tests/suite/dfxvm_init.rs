use crate::common::{
    file_contents::manifest_json, paths::prepend_to_minimal_path, ReleaseAsset, ReleaseServer,
    TempHomeDir,
};
use assert_cmd::prelude::*;
use predicates::boolean::PredicateBooleanExt;
use predicates::str::*;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

// This fake rc copied from https://github.com/rust-lang/rustup/blob/master/tests/suite/cli_paths.rs
// Let's write a fake .rc which looks vaguely like a real script.
pub const FAKE_RC: &str = r#"
# Sources fruity punch.
. ~/fruit/punch

# Adds apples to PATH.
export PATH="$HOME/apple/bin"
"#;

pub fn posix_source() -> String {
    #[cfg(target_os = "macos")]
    let env_path = "$HOME/Library/Application Support/org.dfinity.dfx/env";
    #[cfg(target_os = "linux")]
    let env_path = "$HOME/.local/share/dfx/env";

    format!(". \"{env_path}\"\n", env_path = env_path)
}

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
        .arg("--yes")
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
        .arg("--yes")
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
    let xdg_data_home = home_dir.join(".custom/xdg/data-home");
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
        .arg("--yes")
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

    home_dir.dfxvm_init().arg("--yes").assert().success();

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

#[test]
fn creates_dot_profile() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    home_dir.dfxvm_init().arg("--yes").assert().success();

    let dot_profile = home_dir.join(".profile");
    let contents = std::fs::read_to_string(dot_profile).unwrap();
    assert_eq!(contents, posix_source());
}

#[test]
fn updates_dot_profile() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    let dot_profile = home_dir.join(".profile");
    std::fs::write(&dot_profile, FAKE_RC).unwrap();

    home_dir.dfxvm_init().arg("--yes").assert().success();

    let expected = FAKE_RC.to_owned() + &posix_source();
    let new_dot_profile = std::fs::read_to_string(&dot_profile).unwrap();
    assert_eq!(new_dot_profile, expected);
}

#[test]
fn adds_newline_if_existing_file_does_not_end_in_one() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    let dot_profile = home_dir.join(".profile");
    std::fs::write(&dot_profile, FAKE_RC.trim_end()).unwrap();

    home_dir.dfxvm_init().arg("--yes").assert().success();

    let expected = FAKE_RC.to_owned() + &posix_source();
    let new_dot_profile = std::fs::read_to_string(&dot_profile).unwrap();
    assert_eq!(new_dot_profile, expected);
}

#[test]
fn updates_bash_rcs() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    let rcs: Vec<PathBuf> = [".bashrc", ".bash_profile", ".bash_login"]
        .iter()
        .map(|rc| home_dir.join(rc))
        .collect();
    for rc in &rcs {
        std::fs::write(rc, FAKE_RC).unwrap();
    }

    home_dir.dfxvm_init().arg("--yes").assert().success();

    let expected = FAKE_RC.to_owned() + &posix_source();
    for rc in &rcs {
        let new_rc = std::fs::read_to_string(rc).unwrap();
        assert_eq!(new_rc, expected);
    }
}

#[test]
fn does_not_create_bash_rcs() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);
    server.expect_install_latest();

    let rcs: Vec<PathBuf> = [".bashrc", ".bash_profile", ".bash_login"]
        .iter()
        .map(|rc| home_dir.join(rc))
        .collect();

    home_dir.dfxvm_init().arg("--yes").assert().success();

    for rc in &rcs {
        assert!(!rc.exists(), "{} should not exist", rc.display());
    }
}

#[test]
fn detects_zsh_by_shell_env_var() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);
    server.expect_install_latest();

    let home_zshenv = home_dir.join(".zshenv");

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("SHELL", "/bin/zsh")
        .env("PATH", home_dir.join("nothing"))
        .assert()
        .success();

    assert_eq!(
        std::fs::read_to_string(home_zshenv).unwrap(),
        posix_source()
    );
}

#[test]
fn detects_zsh_by_zsh_on_path() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    let home_zshenv = home_dir.join(".zshenv");

    let bin = tempfile::Builder::new()
        .prefix("dfxvm-integration-tests-bin")
        .tempdir()
        .unwrap();
    // just has to exist
    std::fs::write(bin.path().join("zsh"), "").unwrap();

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("PATH", bin.path())
        .assert()
        .success();

    assert_eq!(
        std::fs::read_to_string(home_zshenv).unwrap(),
        posix_source()
    );
}

#[test]
fn does_not_detect_zsh_by_zdotdir_env_var() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    let zdotdir = home_dir.join("zdotdir");
    std::fs::create_dir(&zdotdir).unwrap();

    // let's even create a zshenv in there
    let zdotdir_zshenv = zdotdir.join(".zshenv");
    std::fs::write(&zdotdir_zshenv, FAKE_RC).unwrap();

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("ZDOTDIR", &zdotdir)
        .env("PATH", home_dir.join("nothing"))
        .assert()
        .success();

    assert_eq!(std::fs::read_to_string(&zdotdir_zshenv).unwrap(), FAKE_RC);
}

#[test]
fn creates_zshenv() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    let home_zshenv = home_dir.join(".zshenv");

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("SHELL", "/bin/zsh")
        .env("PATH", home_dir.join("nothing"))
        .assert()
        .success();

    assert_eq!(
        std::fs::read_to_string(home_zshenv).unwrap(),
        posix_source()
    );
}

#[test]
fn updates_zshenv() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    let home_zshenv = home_dir.join(".zshenv");
    std::fs::write(&home_zshenv, FAKE_RC).unwrap();

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("SHELL", "/bin/zsh")
        .env("PATH", home_dir.join("nothing"))
        .assert()
        .success();

    assert_eq!(
        std::fs::read_to_string(&home_zshenv).unwrap(),
        FAKE_RC.to_owned() + &posix_source()
    );
}

#[test]
fn creates_zdotdir_zshenv() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    let zdotdir = home_dir.join("zdotdir");
    std::fs::create_dir(&zdotdir).unwrap();

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("SHELL", "/bin/zsh")
        .env("ZDOTDIR", &zdotdir)
        .env("PATH", home_dir.join("nothing"))
        .assert()
        .success();

    let zdotdir_zshenv = zdotdir.join(".zshenv");
    assert_eq!(
        std::fs::read_to_string(zdotdir_zshenv).unwrap(),
        posix_source()
    );
}

#[test]
fn updates_zdotdir_zshenv() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    let zdotdir = home_dir.join("zdotdir");
    std::fs::create_dir(&zdotdir).unwrap();
    std::fs::write(zdotdir.join(".zshenv"), FAKE_RC).unwrap();

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("SHELL", "/bin/zsh")
        .env("ZDOTDIR", &zdotdir)
        .env("PATH", home_dir.join("nothing"))
        .assert()
        .success();

    let zdotdir_zshenv = zdotdir.join(".zshenv");
    assert_eq!(
        std::fs::read_to_string(zdotdir_zshenv).unwrap(),
        FAKE_RC.to_owned() + &posix_source()
    );
}

#[test]
fn gets_zdotdir_by_calling_zsh() {
    let home_dir = TempHomeDir::new();
    // this test requires that zsh is callable.
    if home_dir
        .new_command("zsh")
        .arg("-c")
        .arg("true")
        .output()
        .is_err()
    {
        return;
    }

    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    let home_zshenv = home_dir.join(".zshenv");
    let zdotdir = home_dir.join("my-zdot-dir");
    let zdotdir_zshenv = zdotdir.join(".zshenv");

    std::fs::create_dir(&zdotdir).unwrap();
    std::fs::write(zdotdir.join(".zshenv"), FAKE_RC).unwrap();

    let export_zdotdir = format!(
        "export ZDOTDIR={}",
        zdotdir.into_os_string().to_str().unwrap()
    );
    std::fs::write(home_zshenv, export_zdotdir).unwrap();

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("SHELL", "/bin/sh")
        .assert()
        .success();
    //.stderr("abc");

    assert_eq!(
        std::fs::read_to_string(zdotdir_zshenv).unwrap(),
        FAKE_RC.to_owned() + &posix_source()
    );
}

#[test]
fn ignores_empty_zdotdir_env_var() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("SHELL", "/bin/zsh")
        .env("ZDOTDIR", "")
        .env("PATH", home_dir.join("nothing"))
        .assert()
        .success();

    let home_zshenv = home_dir.join(".zshenv");
    assert_eq!(
        std::fs::read_to_string(home_zshenv).unwrap(),
        posix_source()
    );
}

#[test]
fn prefers_zdotdir_to_home_zshenv_if_neither_exist() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    // here ZDOTDIR is set, but neither $ZDOTDIR/.zshenv nor $HOME/.zshenv exist
    // so we should create $ZDOTDIR/.zshenv
    let zdotdir = home_dir.join("the-zdot-dir");
    // dfxvm-init will not create $ZDOTDIR, so we have to
    std::fs::create_dir(&zdotdir).unwrap();

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("SHELL", "/bin/zsh")
        .env("ZDOTDIR", &zdotdir)
        .assert()
        .success();

    let zdotdir_zshenv = zdotdir.join(".zshenv");
    assert_eq!(
        std::fs::read_to_string(zdotdir_zshenv).unwrap(),
        posix_source()
    );
    assert!(!home_dir.join(".zshenv").exists());
}

#[test]
fn prefers_zdotdir_to_home_zshenv_if_both_exist() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    // here ZDOTDIR is set, and both $ZDOTDIR/.zshenv and $HOME/.zshenv exist
    // so we should update $ZDOTDIR/.zshenv
    let zdotdir = home_dir.join("the-zdot-dir");
    let zdotdir_zshenv = zdotdir.join(".zshenv");
    let home_zshenv = home_dir.join(".zshenv");
    std::fs::create_dir(&zdotdir).unwrap();
    std::fs::write(zdotdir_zshenv, FAKE_RC).unwrap();
    std::fs::write(&home_zshenv, "echo this is the home zshenv").unwrap();

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("SHELL", "/bin/zsh")
        .env("ZDOTDIR", &zdotdir)
        .assert()
        .success();

    let zdotdir_zshenv = zdotdir.join(".zshenv");
    assert_eq!(
        std::fs::read_to_string(zdotdir_zshenv).unwrap(),
        FAKE_RC.to_owned() + &posix_source()
    );
    assert_eq!(
        std::fs::read_to_string(home_zshenv).unwrap(),
        "echo this is the home zshenv"
    );
}

#[test]
fn prefers_home_zshenv_if_zdotdir_zshenv_does_not_exist() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    // here ZDOTDIR is set, but $ZDOTDIR/.zshenv does not exist.
    // $HOME/.zshenv does exist, so we should update that.

    let zdotdir = home_dir.join("the-zdot-dir");
    // dfxvm-init will not create $ZDOTDIR, so we have to
    std::fs::create_dir(&zdotdir).unwrap();

    let home_zshenv = home_dir.join(".zshenv");
    std::fs::write(&home_zshenv, FAKE_RC).unwrap();

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("SHELL", "/bin/zsh")
        .env("ZDOTDIR", &zdotdir)
        .assert()
        .success();

    assert_eq!(
        std::fs::read_to_string(home_zshenv).unwrap(),
        FAKE_RC.to_owned() + &posix_source()
    );
    assert!(!zdotdir.join(".zshenv").exists());
}

#[test]
fn confirmation_message_profile_scripts_modified() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    #[cfg(target_os = "macos")]
    let expected_env_path = "$HOME/Library/Application Support/org.dfinity.dfx/env";
    #[cfg(target_os = "linux")]
    let expected_env_path = "$HOME/.local/share/dfx/env";

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .assert()
        .success()
        .stdout(contains("dfxvm is installed now."))
        .stdout(contains(
            "To get started you may need to restart your current shell",
        ))
        .stdout(contains(
            "This would reload your PATH environment variable to include",
        ))
        .stdout(contains("the dfxvm bin directory"))
        .stdout(contains("To configure your shell, run:"))
        .stdout(contains(format!(r#"  source "{expected_env_path}""#)));

    assert!(home_dir.installed_dfx_path("0.15.0").exists());
    assert_eq!(home_dir.settings().read_default_version(), "0.15.0");
}

#[test]
fn confirmation_message_profile_scripts_not_modified() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    server.expect_install_latest();

    #[cfg(target_os = "macos")]
    let expected_env_path = "$HOME/Library/Application Support/org.dfinity.dfx/env";
    #[cfg(target_os = "linux")]
    let expected_env_path = "$HOME/.local/share/dfx/env";

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .arg("--no-modify-path")
        .assert()
        .success()
        .stdout(contains("dfxvm is installed now."))
        .stdout(contains(
            "To get started you need the dfxvm bin directory in your PATH",
        ))
        .stdout(contains("This has not been done automatically"))
        .stdout(contains("To configure your shell, run:"))
        .stdout(contains(format!(r#"  source "{expected_env_path}""#)));

    assert!(home_dir.installed_dfx_path("0.15.0").exists());
    assert_eq!(home_dir.settings().read_default_version(), "0.15.0");
}

#[test]
fn deletes_dfx_on_path() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.15.0", "echo 'this is dfx 0.15.0'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);
    server.expect_get_manifest(&manifest_json("0.15.0"));

    let another_bin_dir = home_dir.join("another-bin-dir");
    let dfx_on_path = another_bin_dir.join("dfx");
    std::fs::create_dir(&another_bin_dir).unwrap();
    std::fs::write(&dfx_on_path, "does not matter").unwrap();

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("PATH", prepend_to_minimal_path(&another_bin_dir))
        .assert()
        .success()
        .stderr(is_match("deleted:.*another-bin-dir/dfx").unwrap());

    assert!(!dfx_on_path.exists());
}

#[test]
fn does_not_delete_dfx_proxy() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.15.0", "echo 'this is dfx 0.15.0'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);
    server.expect_get_manifest(&manifest_json("0.15.0"));

    home_dir.install_dfxvm_bin_as_dfx_proxy();

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env(
            "PATH",
            prepend_to_minimal_path(home_dir.installed_bin_dir()),
        )
        .assert()
        .success()
        .stderr(contains("deleted:").not());
}

#[test]
fn copes_with_nonexistent_dir_on_path() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.15.0", "echo 'this is dfx 0.15.0'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);
    server.expect_get_manifest(&manifest_json("0.15.0"));

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env(
            "PATH",
            prepend_to_minimal_path(home_dir.join("does-not-exist")),
        )
        .assert()
        .success();
}

#[test]
fn removes_dfx_uninstall_script() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let tarball = ReleaseAsset::dfx_tarball("0.15.0", "echo 'this is dfx 0.15.0'");
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);
    server.expect_get_manifest(&manifest_json("0.15.0"));

    std::fs::create_dir_all(home_dir.dfinity_cache_dir()).unwrap();
    let uninstall_script_path = home_dir.dfinity_cache_dir().join("uninstall.sh");
    std::fs::write(&uninstall_script_path, "does not matter").unwrap();
    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env(
            "PATH",
            prepend_to_minimal_path(home_dir.join("does-not-exist")),
        )
        .assert()
        .success();

    assert!(!uninstall_script_path.exists());
}
