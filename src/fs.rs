use crate::error::fs::{
    CanonicalizePathError, CreateDirAllError, CreateFileError, OpenFileError, ReadFileError,
    ReadToStringError, RenameError, WriteFileError,
};
use std::path::{Path, PathBuf};

pub fn canonicalize(path: &Path) -> Result<PathBuf, CanonicalizePathError> {
    path.canonicalize().map_err(|source| CanonicalizePathError {
        path: path.to_path_buf(),
        source,
    })
}

pub fn create_file(path: &Path) -> Result<std::fs::File, CreateFileError> {
    std::fs::File::create(path).map_err(|source| CreateFileError {
        path: path.to_path_buf(),
        source,
    })
}

pub fn open_file(path: &Path) -> Result<std::fs::File, OpenFileError> {
    std::fs::File::open(path).map_err(|source| OpenFileError {
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

pub fn read_to_string(path: &Path) -> Result<String, ReadToStringError> {
    std::fs::read_to_string(path).map_err(|source| ReadToStringError {
        path: path.to_path_buf(),
        source,
    })
}

pub fn rename(from: &Path, to: &Path) -> Result<(), RenameError> {
    std::fs::rename(from, to).map_err(|source| RenameError {
        from: from.to_path_buf(),
        to: to.to_path_buf(),
        source,
    })
}

pub fn create_dir_all(path: &Path) -> Result<(), CreateDirAllError> {
    std::fs::create_dir_all(path).map_err(|source| CreateDirAllError {
        path: path.to_path_buf(),
        source,
    })
}

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<(), WriteFileError> {
    std::fs::write(path.as_ref(), contents).map_err(|source| WriteFileError {
        path: path.as_ref().to_path_buf(),
        source,
    })
}
