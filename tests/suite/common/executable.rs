use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

pub fn create_executable(path: &Path, contents: &str) {
    std::fs::write(path, contents).unwrap();
    set_executable(path);
    wait_until_file_is_not_busy(path);
}

pub fn wait_until_file_is_not_busy(path: &Path) {
    const TEXT_FILE_BUSY: i32 = 26;
    loop {
        match Command::new(path).output() {
            Ok(_) => return,
            Err(err) if matches!(err.raw_os_error(), Some(TEXT_FILE_BUSY)) => {
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(other) => panic!("unexpected error waiting for file to be ready: {other}"),
        }
    }
}

fn set_executable(bin_path: &Path) {
    let mut perms = std::fs::metadata(bin_path).unwrap().permissions();
    perms.set_mode(perms.mode() | 0o500);
    std::fs::set_permissions(bin_path, perms).unwrap();
}
