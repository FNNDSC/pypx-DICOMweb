//! Error definitions.

use std::path::PathBuf;

/// Error with application config.
#[derive(thiserror::Error, Debug)]
#[error("Not a directory: {0:?}")]
pub struct PypxBaseNotADir(pub PathBuf);

/// Error reading file from pypx-organized directory.
#[derive(thiserror::Error, Debug)]
pub enum FileError {
    #[error("Cannot read parent directory: {0:?}")]
    ParentDirNotReadable(PathBuf, std::io::ErrorKind),
    #[error("File not found: {0:?}")]
    NotFound(PathBuf),
    #[error("File content is malformed: {0:?} -- Reason: {1}")]
    Malformed(
        PathBuf,
        String,
        Option<Box<dyn std::error::Error + Send + Sync>>,
    ),
    #[error("Error reading file ({1:?}): {0:?}")]
    IO(PathBuf, std::io::ErrorKind),

    /// Most [FileError::Runtime] errors happen with plumbing that I don't fully understand.
    #[error("Runtime error while processing {0:?} -- {1:?}")]
    Runtime(PathBuf, Box<dyn std::error::Error + Send + Sync>),
}

impl FileError {
    pub(crate) fn from_io_error(path: PathBuf, error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => FileError::NotFound(path),
            _ => FileError::IO(path, error.kind()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Error reading directory (1:?): {0:?}")]
pub struct ReadDirError(pub(crate) PathBuf, pub(crate) std::io::ErrorKind);
