use crate::error::fs::{
    CanonicalizePathError, CopyFileError, CreateDirAllError, CreateFileError, OpenFileError,
    ReadFileError, ReadMetadataError, ReadToStringError, RemoveDirAllError, RemoveFileError,
    RenameError, SetPermissionsError, WriteFileError,
};
use std::path::{Path, PathBuf};

pub fn canonicalize(path: &Path) -> Result<PathBuf, CanonicalizePathError> {
    path.canonicalize().map_err(|source| CanonicalizePathError {
        path: path.to_path_buf(),
        source,
    })
}

pub fn copy(from: &Path, to: &Path) -> Result<u64, CopyFileError> {
    std::fs::copy(from, to).map_err(|source| CopyFileError {
        from: from.to_path_buf(),
        to: to.to_path_buf(),
        source,
    })
}

pub fn create_file(path: &Path) -> Result<std::fs::File, CreateFileError> {
    std::fs::File::create(path).map_err(|source| CreateFileError {
        path: path.to_path_buf(),
        source,
    })
}

pub fn metadata(path: &Path) -> Result<std::fs::Metadata, ReadMetadataError> {
    std::fs::metadata(path).map_err(|source| ReadMetadataError {
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

pub fn remove_dir_all(path: &Path) -> Result<(), RemoveDirAllError> {
    std::fs::remove_dir_all(path).map_err(|source| RemoveDirAllError {
        path: path.to_path_buf(),
        source,
    })
}

pub fn remove_file(path: &Path) -> Result<(), RemoveFileError> {
    std::fs::remove_file(path).map_err(|source| RemoveFileError {
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

pub fn set_permissions(
    path: &Path,
    permissions: std::fs::Permissions,
) -> Result<(), SetPermissionsError> {
    std::fs::set_permissions(path, permissions).map_err(|source| SetPermissionsError {
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
