use crate::error::dfxvm::SelfUninstallError;
use crate::locations::Locations;

pub fn self_uninstall(_locations: &Locations) -> Result<(), SelfUninstallError> {
    println!("uninstall dfxvm and all dfx versions");
    Ok(())
}
