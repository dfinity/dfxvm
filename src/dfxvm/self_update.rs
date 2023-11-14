use crate::error::dfxvm::SelfUpdateError;

pub fn self_update() -> Result<(), SelfUpdateError> {
    println!("update dfxvm to latest");
    Ok(())
}
