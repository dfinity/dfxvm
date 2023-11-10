use crate::error::dfxvm::DefaultError;
use semver::Version;

pub fn default(version: Version) -> Result<(), DefaultError> {
    println!("use dfx {} by default", version);
    Ok(())
}
