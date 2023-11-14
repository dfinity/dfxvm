use crate::error::dfxvm::UninstallError;
use semver::Version;

pub fn uninstall(version: Version) -> Result<(), UninstallError> {
    println!("uninstall dfx {}", version);
    Ok(())
}
