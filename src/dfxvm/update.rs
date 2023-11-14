use crate::error::dfxvm::UpdateError;

pub fn update() -> Result<(), UpdateError> {
    println!("update to latest dfx");
    Ok(())
}
