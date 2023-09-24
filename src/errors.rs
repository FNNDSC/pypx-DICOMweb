//! Error definitions.

use std::path::PathBuf;

/// Error with application config.
#[derive(thiserror::Error, Debug)]
#[error("Not a directory: {0:?}")]
pub struct PypxBaseNotADir(pub PathBuf);

/// Error reading JSON file from pypx-organized directory.
#[derive(thiserror::Error, Debug)]
pub enum JsonFileError {
    #[error("File not found: {0:?}")]
    NotFound(PathBuf),
    #[error("File content is malformed: {0:?}")]
    Malformed(PathBuf),
    #[error("Error reading file ({1:?}): {0:?}")]
    IO(PathBuf, std::io::ErrorKind),
}

impl JsonFileError {
    pub(crate) fn from_io_error(path: PathBuf, error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => JsonFileError::NotFound(path),
            _ => JsonFileError::IO(path, error.kind())
        }
    }
}


#[derive(thiserror::Error, Debug)]
#[error("Error reading directory (1:?): {0:?}")]
pub struct ReadDirError(pub(crate) PathBuf, pub(crate) std::io::ErrorKind);
