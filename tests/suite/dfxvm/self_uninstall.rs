use crate::common::file_contents::manifest_json;
use crate::common::{ReleaseAsset, ReleaseServer, TempHomeDir};
use crate::dfxvm_init::{posix_source, FAKE_RC};
use assert_cmd::prelude::*;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn self_uninstall() {
    let home_dir = TempHomeDir::new();
    let server = ReleaseServer::new(&home_dir);

    let fake_dfx = looping_executable();

    let tarball = ReleaseAsset::dfx_tarball_with_dfx_contents("0.15.0", &fake_dfx);
    let sha256 = ReleaseAsset::sha256(&tarball);
    server.expect_get(&tarball);
    server.expect_get(&sha256);
    server.expect_get_manifest(&manifest_json("0.15.0"));

    let all_rcs = [
        ".zshenv",
        ".profile",
        ".bash_profile",
        ".bash_login",
        ".bashrc",
    ];

    for rc in all_rcs {
        let rc_path = home_dir.join(rc);
        std::fs::write(&rc_path, FAKE_RC).unwrap();
    }

    home_dir
        .dfxvm_init()
        .arg("--yes")
        .env("SHELL", "zsh") // force .zshenv update
        .assert()
        .success();

    populate_dfinity_cache(&home_dir, &fake_dfx);
    populate_local_network_dir(&home_dir);

    std::fs::write(home_dir.installed_bin_dir().join("junk"), "junk").unwrap();

    let mut h = Command::new(home_dir.dfx_version_path("0.15.0"))
        .spawn()
        .unwrap();

    assert!(home_dir.installed_dfxvm_path().exists());
    assert!(home_dir.installed_dfx_proxy_path().exists());
    assert!(home_dir.versions_dir().exists());
    assert!(home_dir.installed_env_path().exists());
    for rc in all_rcs {
        let rc_path = home_dir.join(rc);
        let rc = std::fs::read_to_string(&rc_path).unwrap();
        let expected = FAKE_RC.to_owned() + &posix_source();
        assert_eq!(rc, expected, "rc: {}", rc_path.display());
    }
    assert!(home_dir.dfinity_cache_versions_dir().exists());
    assert!(all_process_exe_paths().contains(&home_dir.dfx_version_path("0.15.0")));

    home_dir
        .installed_dfxvm()
        .args(["self", "uninstall", "--yes"])
        .assert()
        .success();

    // deletes bin/dfxvm
    assert!(!home_dir.installed_dfxvm_path().exists());
    assert!(!home_dir.installed_dfx_proxy_path().exists());
    assert!(!home_dir.installed_bin_dir().exists());
    assert!(!home_dir.versions_dir().exists());
    assert!(!home_dir.installed_env_path().exists());
    for rc in all_rcs {
        let rc_path = home_dir.join(rc);
        let rc = std::fs::read_to_string(&rc_path).unwrap();
        let expected = FAKE_RC.to_owned() + "\n";
        assert_eq!(rc, expected);
    }
    assert!(!home_dir.dfinity_cache_dir().exists());
    assert!(!all_process_exe_paths().contains(&home_dir.dfx_version_path("0.15.0")));
    assert!(!home_dir.data_local_dir().exists());

    h.wait().unwrap();
}

fn populate_local_network_dir(temp_home_dir: &TempHomeDir) {
    let dir = temp_home_dir.data_local_dir().join("network").join("local");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("webserver-port"), "7654").unwrap();
}

fn populate_dfinity_cache(home_dir: &TempHomeDir, fake_dfx: &[u8]) {
    std::fs::create_dir_all(home_dir.cache_pulled_path()).unwrap();
    std::fs::write(home_dir.cache_pulled_path().join("a-file"), "whatever").unwrap();
    std::fs::create_dir_all(home_dir.cache_downloads_path()).unwrap();
    std::fs::write(
        home_dir.cache_downloads_path().join("dfx-0.7.2.tar.gz"),
        "ok",
    )
    .unwrap();
    std::fs::write(
        home_dir.legacy_uninstall_script_path(),
        "#!/usr/bin/env bash\necho uninstall",
    )
    .unwrap();
    populate_dfx_cache_version("0.15.0", home_dir, fake_dfx);
    populate_dfx_cache_version("0.14.4", home_dir, fake_dfx);
}

fn populate_dfx_cache_version(version: &str, home_dir: &TempHomeDir, fake_dfx: &[u8]) {
    let dir = home_dir.dfinity_cache_versions_dir().join(version);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("dfx"), fake_dfx).unwrap();
}

fn all_process_exe_paths() -> Vec<PathBuf> {
    let mut info = sysinfo::System::new();
    info.refresh_processes();
    info.processes()
        .iter()
        .filter_map(|(_pid, proc)| proc.exe().map(|p| p.to_path_buf()))
        .collect()
}

fn looping_executable() -> Vec<u8> {
    let tempdir = tempfile::Builder::new()
        .prefix("dfxvm-integration-tests-builder")
        .tempdir()
        .unwrap();

    let cargo_toml = r#"
[package]
name = "loop"
version = "0.1.0"
edition = "2018"

[[bin]]
name = "loop"
path = "main.rs"
"#;
    let main_rs = r#"
fn main() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
"#;

    std::fs::write(tempdir.path().join("Cargo.toml"), cargo_toml).unwrap();
    std::fs::write(tempdir.path().join("main.rs"), main_rs).unwrap();

    Command::new("cargo")
        .arg("build")
        .current_dir(tempdir.path())
        .assert()
        .success();

    let path = tempdir.path().join("target").join("debug").join("loop");
    std::fs::read(path).unwrap()
}
