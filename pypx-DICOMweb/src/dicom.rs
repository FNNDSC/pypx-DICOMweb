//! Helper functions for reading DICOM files.

use crate::errors::FileError;
use dicom::object::ReadError;
use dicom::pixeldata::PixelDecoder;
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
            dicom_json::to_value(dcm).map_err(|error| {
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

/// Get a frame (zero-indexed) of a DICOM file.
pub async fn encode_frame(path: PathBuf, frame: u32) -> Result<Vec<u8>, FileError> {
    let p = path.to_path_buf();
    tokio::task::spawn_blocking(move || encode_frame_sync(p, frame))
        .await
        .map_err(|error| {
            event!(Level::ERROR, "Don't know what just happened! {:?}", error);
            FileError::Malformed(path.to_path_buf())
        })?
}

fn encode_frame_sync(path: PathBuf, frame: u32) -> Result<Vec<u8>, FileError> {
    let dcm = dicom::object::open_file(&path).map_err(|error| convert_error(&path, error))?;
    let pixel_data = dcm.decode_pixel_data().map_err(|error| {
        event!(
            Level::ERROR,
            "Could not read pixel data of {:?}: {:?}",
            &path,
            error
        );
        FileError::Malformed(path.to_path_buf())
    })?;
    // Previously in commit 4a2646f0260bc72530abb3f163c112cb7e51481b
    // I was encoding the data as JPEG, which would cause glitches in OHIF.
    // OHIF seems to have the best support for image/jls and raw DICOM pixel data.
    pixel_data
        .frame_data(frame)
        .map(|data| data.to_vec())
        .map_err(|error| {
            event!(
                Level::ERROR,
                "Failed to get data from {:?} at frame={}: {:?}",
                &path,
                frame,
                error
            );
            FileError::Malformed(path)
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
