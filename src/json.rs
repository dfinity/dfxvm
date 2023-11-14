use crate::error::json::{LoadJsonFileError, SaveJsonFileError};
use crate::fs::read;
use serde::Serialize;
use std::path::Path;

pub fn load_json_file<T: for<'a> serde::de::Deserialize<'a>>(
    path: &Path,
) -> Result<T, LoadJsonFileError> {
    let content = read(path)?;

    serde_json::from_slice(content.as_ref()).map_err(|source| LoadJsonFileError::Parse {
        path: path.to_path_buf(),
        source,
    })
}

pub fn save_json_file<T: Serialize>(path: &Path, value: &T) -> Result<(), SaveJsonFileError> {
    let content =
        serde_json::to_string_pretty(&value).map_err(|source| SaveJsonFileError::Serialize {
            path: path.to_path_buf(),
            source,
        })?;
    crate::fs::write(path, content)?;
    Ok(())
}
