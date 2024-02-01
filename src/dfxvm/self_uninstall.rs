use crate::error::{
    dfxvm::{self_uninstall::UninstallProfileScriptsError, SelfUninstallError},
    dfxvm_init::InteractError,
    fs::{RemoveDirAllError, RemoveFileError},
};
use crate::fs::{read, remove_dir_all, remove_file, write};
use crate::installation::get_all_profile_scripts;
use crate::locations::Locations;
use console::style;
use dialoguer::Confirm;
use std::path::Path;
use sysinfo::System;

pub fn self_uninstall(yes: bool, locations: &Locations) -> Result<(), SelfUninstallError> {
    println!();
    println!("Thanks for developing with dfx and dfxvm!");
    println!();
    println!("This will uninstall dfxvm and all dfx versions, and remove");
    println!(
        "{} from your PATH environment variable.",
        style(locations.bin_dir().display()).bold()
    );
    println!();

    if !yes && !confirm()? {
        info!("canceling uninstallation");
        return Ok(());
    }

    killall_dfx(locations);
    delete_dir(&locations.network_dir())?;
    delete_dir(locations.dfinity_cache_dir())?;
    delete_dir(locations.versions_dir())?;
    delete_file(&locations.dfx_proxy_path())?;
    uninstall_from_profile_scripts()?;
    delete_file(&locations.env_path())?;
    delete_own_binary(locations)?;
    remove_dir_all(&locations.bin_dir())?;
    remove_dir_all(locations.data_local_dir())?;

    if locations.config_dir().exists() {
        info!("did not delete the config directory because it may contain identities.");
        info!("config directory: {}", locations.config_dir().display());
    }

    Ok(())
}

fn delete_own_binary(locations: &Locations) -> Result<(), RemoveFileError> {
    delete_file(&locations.dfxvm_path())
}

fn uninstall_from_profile_scripts() -> Result<(), UninstallProfileScriptsError> {
    for ps in get_all_profile_scripts() {
        if ps.is_file() {
            let source_bytes = ps.source_string().into_bytes();
            let file_bytes = read(&ps.path)?;
            // This approach copied from https://github.com/rust-lang/rustup/blob/master/src/cli/self_update/unix.rs#L56
            if let Some(idx) = file_bytes
                .windows(source_bytes.len())
                .position(|w| w == source_bytes.as_slice())
            {
                // Here we rewrite the file without the offending line.
                let mut new_bytes = file_bytes[..idx].to_vec();
                new_bytes.extend(&file_bytes[idx + source_bytes.len()..]);
                write(&ps.path, &new_bytes)?;
            }
        }
    }
    Ok(())
}

fn killall_dfx(locations: &Locations) {
    while killany_dfx(locations) {
        continue;
    }
}

fn killany_dfx(locations: &Locations) -> bool {
    let versions_dir = locations.versions_dir();
    let dfinity_cache_versions_dir = locations.dfinity_cache_versions_dir();

    let mut info = System::new();
    info.refresh_processes();
    let mut n = 0;
    for (pid, proc) in info.processes() {
        if let Some(exe) = proc.exe() {
            if exe.starts_with(versions_dir) || exe.starts_with(&dfinity_cache_versions_dir) {
                info!("killing {} {}", pid.as_u32(), exe.display());
                n += 1;
                proc.kill();
            }
        }
    }
    n > 0
}

fn confirm() -> Result<bool, InteractError> {
    let uninstall = Confirm::new()
        .with_prompt("Continue?")
        .default(false)
        .interact()?;
    Ok(uninstall)
}

fn delete_dir(dir: &Path) -> Result<(), RemoveDirAllError> {
    if dir.exists() {
        info!("deleting {}", dir.display());
        remove_dir_all(dir)?;
    }
    Ok(())
}

fn delete_file(path: &Path) -> Result<(), RemoveFileError> {
    if path.exists() {
        info!("deleting {}", path.display());
        remove_file(path)?;
    }
    Ok(())
}
