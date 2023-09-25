//! Helper functions for reading DICOM files.

use crate::errors::FileError;
use dicom::object::ReadError;
use dicom::pixeldata::PixelDecoder;
use serde_json::Value;
use std::path::{Path, PathBuf};

/// Serialize DICOM file as JSON.
pub(crate) async fn dicomfile2json(path: PathBuf) -> Result<Value, FileError> {
    let p = path.to_path_buf();
    tokio::task::spawn_blocking(move || dicom::object::open_file(p))
        .await
        .map_err(|error| FileError::Runtime(path.to_path_buf(), error.into()))?
        .map_err(|error| convert_error(&path, error))
        .and_then(|dcm| {
            dicom_json::to_value(dcm).map_err(|error| {
                FileError::Malformed(
                    path,
                    "Could not parse as JSON".to_string(),
                    Some(error.into()),
                )
            })
        })
}

/// Get a frame (zero-indexed) of a DICOM file.
pub async fn encode_frame(path: PathBuf, frame: u32) -> Result<Vec<u8>, FileError> {
    let p = path.to_path_buf();
    tokio::task::spawn_blocking(move || encode_frame_sync(p, frame))
        .await
        .map_err(|error| FileError::Runtime(path.to_path_buf(), error.into()))?
}

fn encode_frame_sync(path: PathBuf, frame: u32) -> Result<Vec<u8>, FileError> {
    let dcm = dicom::object::open_file(&path).map_err(|error| convert_error(&path, error))?;
    let pixel_data = dcm.decode_pixel_data().map_err(|error| {
        FileError::Malformed(
            path.to_path_buf(),
            "Could not decode pixel data".to_string(),
            Some(error.into()),
        )
    })?;
    // Previously in commit 4a2646f0260bc72530abb3f163c112cb7e51481b
    // I was encoding the data as JPEG, which would cause glitches in OHIF.
    // OHIF seems to have the best support for image/jls and raw DICOM pixel data.
    pixel_data
        .frame_data(frame)
        .map(|data| data.to_vec())
        .map_err(|error| {
            FileError::Malformed(
                path,
                format!("Failed to get pixel data at frame={frame}"),
                Some(error.into()),
            )
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
                FileError::IO(filename, source.kind())
            }
        }
        _ => FileError::Malformed(
            path.to_path_buf(),
            "Unable to read DICOM file".to_string(),
            None,
        ),
    }
}
