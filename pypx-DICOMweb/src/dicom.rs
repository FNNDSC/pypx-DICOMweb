//! Helper functions for reading DICOM files.

use crate::errors::FileError;
use dicom::object::ReadError;
use serde_json::Value;
use std::path::{Path, PathBuf};
use tracing::{event, Level};

/// Serialize DICOM file as JSON.
pub(crate) async fn dicomfile2json(path: PathBuf) -> Result<Value, FileError> {
    let p = path.to_path_buf();
    tokio::task::spawn_blocking(move || dicom::object::open_file(p))
        .await
        .map_err(|error| {
            event!(Level::ERROR, "Don't know what just happened! {:?}", error);
            FileError::Malformed(path.to_path_buf())
        })?
        .map_err(|error| convert_error(&path, error))
        .and_then(|dcm| {
            dicom_json::to_value(dcm)
                .map_err(|error| {
                    event!(
                        Level::ERROR,
                        "Failed to serialize DICOM file {:?} as JSON: {:?}",
                        &path,
                        error
                    );
                    FileError::Malformed(path)
                })
        })
}

fn convert_error(path: &Path, error: ReadError) -> FileError {
    match error {
        ReadError::OpenFile {
            filename, source, ..
        } => {
            if matches!(source.kind(), std::io::ErrorKind::NotFound) {
                FileError::NotFound(filename)
            } else {
                event!(Level::ERROR, "Unable to open DICOM file: {:?}", path);
                FileError::IO(filename, source.kind())
            }
        }
        _ => {
            event!(Level::ERROR, "Unable to read DICOM file: {:?}", path);
            FileError::Malformed(path.to_path_buf())
        }
    }
}
