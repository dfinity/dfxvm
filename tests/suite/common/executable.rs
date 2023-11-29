use backoff::{retry, ExponentialBackoff};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

pub fn create_executable(path: &Path, contents: &str) {
    std::fs::write(path, contents).unwrap();
    set_executable(path);

    let backoff = ExponentialBackoff::default();
    retry(backoff, || {
        let mut command = Command::new(path);
        let result = command.output();

        const TEXT_FILE_BUSY: i32 = 26;
        match result {
            Ok(output) => Ok(output),
            Err(err) if matches!(err.raw_os_error(), Some(TEXT_FILE_BUSY)) => {
                Err(backoff::Error::transient(err))
            }
            Err(other) => Err(backoff::Error::permanent(other)),
        }
    })
    .unwrap();
}

fn set_executable(bin_path: &Path) {
    let mut perms = std::fs::metadata(bin_path).unwrap().permissions();
    perms.set_mode(perms.mode() | 0o500);
    std::fs::set_permissions(bin_path, perms).unwrap();
}
