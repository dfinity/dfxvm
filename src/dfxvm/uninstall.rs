use crate::dfxvm::install::installed;
use crate::error::dfxvm::UninstallError;
use crate::fs::{remove_dir_all, remove_file, rename};
use crate::locations::Locations;
use semver::Version;

pub fn uninstall(version: Version, locations: &Locations) -> Result<(), UninstallError> {
    let version_dir = locations.version_dir(&version);
    if !installed(&version, locations) {
        info!("dfx {} is not installed", version);
        return Ok(());
    }

    info!("uninstalling dfx {}", version);
    let uninstall_dir = locations
        .versions_dir()
        .join(format!(".uninstall-{version}"));
    if uninstall_dir.exists() {
        if uninstall_dir.is_dir() {
            remove_dir_all(&uninstall_dir)?;
        } else {
            remove_file(&uninstall_dir)?;
        }
    }
    rename(&version_dir, &uninstall_dir)?;
    remove_dir_all(&uninstall_dir)?;
    info!("uninstalled dfx {}", version);
    Ok(())
}
