use crate::error::fs::{CanonicalizePathError, ReadFileError};
use std::path::{Path, PathBuf};

pub fn canonicalize(path: &Path) -> Result<PathBuf, CanonicalizePathError> {
    path.canonicalize().map_err(|source| CanonicalizePathError {
        path: path.to_path_buf(),
        source,
    })
}

pub fn read(path: &Path) -> Result<Vec<u8>, ReadFileError> {
    std::fs::read(path).map_err(|source| ReadFileError {
        path: path.to_path_buf(),
        source,
    })
}
