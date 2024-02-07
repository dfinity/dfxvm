use crate::common::paths::prepend_to_minimal_path;
use crate::common::TempHomeDir;
use assert_cmd::prelude::*;
use predicates::str::*;
use std::os::unix::fs::PermissionsExt;

#[test]
fn no_version_anywhere() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();

    cmd.assert()
        .failure()
        .stdout("")
        .stderr(contains(
            "error: Unable to determine which dfx version to call. To set a default version, run",
        ))
        .stderr(contains("dfxvm default <version>"));
}

#[test]
fn no_version_but_empty_settings() {
    let home_dir = TempHomeDir::new();
    home_dir.settings().write("{}");

    let mut cmd = home_dir.dfx();

    cmd.assert()
        .failure()
        .stdout("")
        .stderr(contains(
            "error: Unable to determine which dfx version to call. To set a default version, run",
        ))
        .stderr(contains("dfxvm default <version>"));
}

#[test]
fn version_from_commandline() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();

    home_dir.create_executable_dfx_script(
        "0.4.2",
        "echo 'this is the zero point four point two dfx executable'",
    );

    cmd.arg("+0.4.2");
    cmd.assert()
        .success()
        .stdout("this is the zero point four point two dfx executable\n");
}

#[test]
fn version_from_environment() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();
    home_dir.create_executable_dfx_script(
        "0.6.7",
        "echo 'this is the zero point six point seven dfx executable'",
    );

    cmd.env("DFX_VERSION", "0.6.7");
    cmd.assert()
        .success()
        .stdout("this is the zero point six point seven dfx executable\n");
}

#[test]
fn version_from_dfx_json() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script(
        "0.7.8",
        "echo 'this is the zero point seven point eight dfx executable'",
    );

    let mut cmd = home_dir.dfx();

    let tempdir = home_dir.new_project_temp_dir();
    let dfx_json = tempdir.path().join("dfx.json");
    std::fs::write(dfx_json, r#"{"dfx": "0.7.8"}"#).unwrap();
    cmd.current_dir(&tempdir);

    cmd.assert()
        .success()
        .stdout("this is the zero point seven point eight dfx executable\n");
}

#[test]
fn version_from_settings() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();

    home_dir.create_executable_dfx_script(
        "0.5.1",
        "echo 'this is the zero point five point one dfx executable'",
    );

    home_dir.settings().write_default_version("0.5.1");

    cmd.assert()
        .success()
        .stdout("this is the zero point five point one dfx executable\n");
}

#[test]
fn version_from_command_line_takes_precedence_over_environment() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();

    home_dir.create_executable_dfx_script(
        "0.4.5",
        "echo 'this is the zero point four point five dfx executable'",
    );
    home_dir.create_executable_dfx_script("0.9.9", "echo 'fail'");

    cmd.arg("+0.4.5");
    cmd.env("DFX_VERSION", "0.9.9");

    cmd.assert()
        .success()
        .stdout("this is the zero point four point five dfx executable\n");
}

#[test]
fn version_from_environment_takes_precedence_over_dfx_json() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script(
        "0.6.7",
        "echo 'this is the zero point six point seven dfx executable'",
    );
    home_dir.create_executable_dfx_script("0.9.9", "echo 'fail'");

    let mut cmd = home_dir.dfx();

    let tempdir = home_dir.new_project_temp_dir();
    let dfx_json = tempdir.path().join("dfx.json");
    std::fs::write(dfx_json, r#"{"dfx": "0.9.9"}"#).unwrap();
    cmd.current_dir(&tempdir);

    cmd.env("DFX_VERSION", "0.6.7");

    cmd.assert()
        .success()
        .stdout("this is the zero point six point seven dfx executable\n");
}

#[test]
fn version_from_dfx_json_takes_precedence_over_settings() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script(
        "0.7.4",
        "echo 'this is the zero point seven point four dfx executable'",
    );
    home_dir.create_executable_dfx_script("0.9.9", "echo 'fail'");
    home_dir.settings().write_default_version("0.9.9");

    let mut cmd = home_dir.dfx();

    let tempdir = home_dir.new_project_temp_dir();
    let dfx_json = tempdir.path().join("dfx.json");
    std::fs::write(dfx_json, r#"{"dfx": "0.7.4"}"#).unwrap();
    cmd.current_dir(&tempdir);

    cmd.assert()
        .success()
        .stdout("this is the zero point seven point four dfx executable\n");
}

#[test]
fn dfx_json_with_no_version() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script(
        "0.2.3",
        "echo 'this is the zero point two point three dfx executable'",
    );
    home_dir.settings().write_default_version("0.2.3");

    let mut cmd = home_dir.dfx();

    let tempdir = home_dir.new_project_temp_dir();
    let dfx_json = tempdir.path().join("dfx.json");
    std::fs::write(dfx_json, r#"{}"#).unwrap();
    cmd.current_dir(&tempdir);

    cmd.assert()
        .success()
        .stdout("this is the zero point two point three dfx executable\n");
}

#[test]
fn version_from_dfx_json_ignores_other_fields() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();
    home_dir.create_executable_dfx_script(
        "0.7.8",
        "echo 'this is the zero point seven point eight dfx executable'",
    );

    let tempdir = home_dir.new_project_temp_dir();
    let dfx_json = tempdir.path().join("dfx.json");
    std::fs::write(dfx_json, r#"{"dfx": "0.7.8", "other": "ignored"}"#).unwrap();
    cmd.current_dir(&tempdir);

    cmd.assert()
        .success()
        .stdout("this is the zero point seven point eight dfx executable\n");
}

#[test]
fn searches_for_dfx_json_in_parent_directories() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();
    home_dir.create_executable_dfx_script(
        "0.7.9",
        "echo 'this is the zero point seven point nine dfx executable'",
    );

    let tempdir = home_dir.new_project_temp_dir();
    let x = tempdir.path().join("x");
    let y = x.join("y");
    let z = y.join("z");
    std::fs::create_dir_all(&z).unwrap();

    let dfx_json = x.join("dfx.json");
    std::fs::write(dfx_json, r#"{"dfx": "0.7.9"}"#).unwrap();

    cmd.current_dir(&z);

    cmd.assert()
        .success()
        .stdout("this is the zero point seven point nine dfx executable\n");
}

#[test]
fn passes_through_exit_code() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script(
        "0.3.2",
        "if [ \"$1\" = \"A\" ]; then exit 7; else exit 9; fi",
    );
    home_dir.settings().write_default_version("0.3.2");

    let mut cmd = home_dir.dfx();
    cmd.arg("A");
    cmd.assert().code(7);

    let mut cmd = home_dir.dfx();
    cmd.arg("B");
    cmd.assert().code(9);
}

#[test]
fn passes_through_stdout_and_stderr() {
    let home_dir = TempHomeDir::new();
    home_dir.create_executable_dfx_script(
        "0.4.3",
        "echo 'hi this is dfx on stdout' ; echo 'hi this is dfx on stderr' >&2",
    );
    home_dir.settings().write_default_version("0.4.3");
    let mut cmd = home_dir.dfx();
    cmd.assert()
        .success()
        .stdout("hi this is dfx on stdout\n")
        .stderr("hi this is dfx on stderr\n");
}

#[test]
fn passes_parameters_not_including_version() {
    let home_dir = TempHomeDir::new();
    let script = r#"
        index=0
        for i in "$@"; do
            echo "param $index: '$i'"
            index=$((index + 1))
        done"#;
    home_dir.create_executable_dfx_script("0.4.4", script);
    let mut cmd = home_dir.dfx();
    cmd.arg("+0.4.4").arg("A").arg("BC").arg("D");
    cmd.assert()
        .success()
        .stdout("param 0: 'A'\nparam 1: 'BC'\nparam 2: 'D'\n");
}

#[test]
fn version_reports_dfx_version() {
    let home_dir = TempHomeDir::new();
    home_dir.settings().write_default_version("0.4.4");

    // make sure it really reports what dfx says it is:
    home_dir.create_executable_dfx_script(
        "0.4.4",
        "if [ \"$1\" = \"--version\" ]; then echo 'dfx 0.8.7'; else echo 'hi this is dfx'; fi",
    );

    let mut cmd = home_dir.dfx();
    cmd.arg("--version");
    cmd.assert().success().stdout("dfx 0.8.7\n").stderr("");
}

#[test]
fn bad_permissions_dfx_binary() {
    let home_dir = TempHomeDir::new();
    home_dir.settings().write_default_version("0.3.1");

    let dfx_binary = home_dir.create_executable_dfx_script("0.3.1", "echo fail");

    let mut perms = std::fs::metadata(&dfx_binary).unwrap().permissions();
    perms.set_mode(0o400);
    std::fs::set_permissions(&dfx_binary, perms).unwrap();

    let mut cmd = home_dir.dfx();
    cmd.arg("--version");
    cmd.assert()
        .failure()
        .stderr(contains(dfx_binary.display().to_string()))
        .stderr(contains("Permission denied"));
}

#[test]
fn malformed_version_from_commandline() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();

    cmd.arg("+3.x");
    cmd.assert()
        .failure()
        .stderr(contains("failed to parse version '3.x' from commandline"))
        .stderr(is_match("caused by: .*minor version number").unwrap());
}

#[test]
fn malformed_version_from_environment() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();

    cmd.env("DFX_VERSION", "3.x");
    cmd.assert()
        .failure()
        .stderr(contains(
            "failed to parse DFX_VERSION '3.x' from environment",
        ))
        .stderr(is_match("caused by: .*minor version number").unwrap());
}

#[test]
fn malformed_json_in_dfx_json() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();

    let tempdir = home_dir.new_project_temp_dir();
    let dfx_json = tempdir.path().join("dfx.json");
    std::fs::write(dfx_json, r#"{ not valid json }"#).unwrap();
    cmd.current_dir(&tempdir);

    cmd.assert()
        .failure()
        .stderr(is_match("failed to parse .*/dfx.json as json").unwrap())
        .stderr(is_match("caused by: .*key must be a string").unwrap());
}

#[test]
fn malformed_version_in_dfx_json() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();

    let tempdir = home_dir.new_project_temp_dir();
    let dfx_json = tempdir.path().join("dfx.json");
    std::fs::write(dfx_json, r#"{"dfx": "3.x"}"#).unwrap();
    cmd.current_dir(&tempdir);

    cmd.assert()
        .failure()
        .stderr(is_match("failed to parse .*/dfx.json as json").unwrap())
        .stderr(is_match("caused by: .*minor version number").unwrap());
}

#[test]
fn malformed_json_in_settings() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();

    home_dir.settings().write("{ not valid json }");

    cmd.assert()
        .failure()
        .stderr(is_match("failed to parse .*/\\.config/dfx/version-manager.json as json").unwrap())
        .stderr(is_match("caused by: .*key must be a string").unwrap());
}

#[test]
fn malformed_version_in_settings() {
    let home_dir = TempHomeDir::new();
    let mut cmd = home_dir.dfx();

    home_dir.settings().write_default_version("X.2");

    cmd.assert()
        .failure()
        .stderr(is_match("failed to parse .*/.config/dfx/version-manager.json as json").unwrap())
        .stderr(is_match("caused by: .*major version number").unwrap());
}

#[test]
fn ignores_empty_version_from_environment() {
    let home_dir = TempHomeDir::new();
    home_dir.settings().write_default_version("0.2.1");
    home_dir.create_executable_dfx_script(
        "0.2.1",
        "echo 'this is the zero point two point one dfx executable'",
    );

    for s in ["", "\n", "\t", "   \n\n\t  "] {
        let mut cmd = home_dir.dfx();
        cmd.env("DFX_VERSION", s);
        cmd.assert()
            .success()
            .stdout("this is the zero point two point one dfx executable\n");
    }
}

#[test]
fn dfx_upgrade_disallowed() {
    let home_dir = TempHomeDir::new();
    home_dir.settings().write_default_version("0.4.4");

    // make sure it really reports what dfx says it is:
    home_dir.create_executable_dfx_script(
        "0.4.4",
        "if [ \"$1\" = \"--version\" ]; then echo 'dfx 0.8.7'; else echo 'hi this is dfx'; fi",
    );

    // upgrade command is disallowed
    home_dir
        .dfx()
        .arg("-v")
        .arg("upgrade")
        .assert()
        .failure()
        .stderr(contains(
            "The command `dfx upgrade` doesn't work with dfxvm",
        ))
        .stderr(contains("dfxvm update"));

    // parameter value doesn't hide it
    home_dir
        .dfx()
        .arg("--identity")
        .arg("me")
        .arg("upgrade")
        .assert()
        .failure()
        .stderr(contains(
            "The command `dfx upgrade` doesn't work with dfxvm",
        ))
        .stderr(contains("dfxvm update"));

    // doesn't block on known parameters whose value technically could be "upgrade"
    home_dir
        .dfx()
        .arg("--identity")
        .arg("upgrade")
        .arg("--logfile")
        .arg("upgrade")
        .arg("--network")
        .arg("upgrade")
        .arg("cache")
        .arg("show")
        .assert()
        .success();

    // upgrade subcommands (could be an extension) are allowed
    home_dir
        .dfx()
        .arg("something")
        .arg("upgrade")
        .assert()
        .success();
}

#[test]
fn env_vars_provided() {
    let home_dir = TempHomeDir::new();
    home_dir
        .create_executable_dfx_script("0.7.9", "echo DFX_VERSION: $DFX_VERSION\necho PATH: $PATH");
    let dfx_version_dir = home_dir.dfx_version_dir("0.7.9");

    let mut cmd = home_dir.dfx();
    cmd.arg("+0.7.9");

    let tempdir = home_dir.new_project_temp_dir();
    let dfx_json = tempdir.path().join("dfx.json");
    std::fs::write(dfx_json, r#"{"dfx": "0.6.2"}"#).unwrap();
    cmd.current_dir(&tempdir);

    let expected_path = prepend_to_minimal_path(dfx_version_dir);

    cmd.assert()
        .success()
        .stdout(contains("DFX_VERSION: 0.7.9\n"))
        .stdout(contains(format!("PATH: {}\n", expected_path)));
}
