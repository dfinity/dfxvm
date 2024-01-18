use crate::error::dfxvm::SelfUpdateError;
use crate::locations::Locations;

pub fn self_update(_locations: &Locations) -> Result<(), SelfUpdateError> {
    println!("update dfxvm to latest");
    Ok(())
}
