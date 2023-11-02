use crate::error::json::LoadJsonFileError;
use crate::fs::read;
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
