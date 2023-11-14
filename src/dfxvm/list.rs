use crate::error::dfxvm::ListError;

pub fn list() -> Result<(), ListError> {
    println!("list installed dfx versions");
    Ok(())
}
