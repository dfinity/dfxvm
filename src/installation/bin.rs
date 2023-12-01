use crate::env::current_exe;
use crate::error::installation::{InstallBinariesError, MakeExecutableError};
use crate::fs::{copy, metadata, remove_file, rename, set_permissions};
use std::fs::hard_link;
use std::path::Path;

pub fn install_binaries(bin_dir: &Path) -> Result<(), InstallBinariesError> {
    let current_exe_path = current_exe()?;
    let installing_dfxvm_init_path = bin_dir.join("dfxvm-init");
    let installed_dfxvm_path = bin_dir.join("dfxvm");
    let installed_dfx_path = bin_dir.join("dfx");

    // copy current exe into bin dir
    if installing_dfxvm_init_path.exists() {
        remove_file(&installing_dfxvm_init_path)?;
    }
    copy(&current_exe_path, &installing_dfxvm_init_path)?;
    make_executable(&installing_dfxvm_init_path)?;

    // rename dfxvm into place
    if installed_dfxvm_path.exists() {
        remove_file(&installed_dfxvm_path)?;
    }
    rename(&installing_dfxvm_init_path, &installed_dfxvm_path)?;

    // create hardlink from dfx to dfxvm, or copy if hardlinking fails
    if installed_dfx_path.exists() {
        remove_file(&installed_dfx_path)?;
    }
    if hard_link(&installed_dfxvm_path, &installed_dfx_path).is_err() {
        copy(&installed_dfxvm_path, &installed_dfx_path)?;
    }
    Ok(())
}

#[cfg(unix)]
fn make_executable(path: &Path) -> Result<(), MakeExecutableError> {
    use std::os::unix::fs::PermissionsExt;

    let metadata = metadata(path)?;
    let mut perms = metadata.permissions();
    let mode = perms.mode();
    let new_mode = (mode & !0o777) | 0o755;

    // Check if permissions are ok already
    if mode == new_mode {
        return Ok(());
    }

    perms.set_mode(new_mode);
    set_permissions(path, perms)?;

    Ok(())
}
