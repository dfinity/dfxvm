use crate::error::dfxvm::SelfUninstallError;

pub fn self_uninstall() -> Result<(), SelfUninstallError> {
    println!("uninstall dfxvm and all dfx versions");
    Ok(())
}
