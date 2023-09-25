//! Helper functions for reading DICOM files.

use crate::errors::FileError;
use dicom::object::ReadError;
use dicom::pixeldata::PixelDecoder;
use serde_json::Value;
use std::io::Cursor;
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

/// Get a frame (zero-indexed) of a DICOM file as a JPG.
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
    let frame_image = pixel_data
        .to_dynamic_image(frame)
        // JPEG only supports 8-bit
        // https://github.com/image-rs/image/issues/1777#issuecomment-1224014721
        .map(|image| image.to_rgba8())
        .map_err(|error| {
            event!(
                Level::ERROR,
                "Could not convert pixel data of {:?} to dynamic image: {:?}",
                &path,
                error
            );
            FileError::Malformed(path.to_path_buf())
        })?;
    let mut buf = Cursor::new(Vec::with_capacity(std::mem::size_of_val(&frame_image)));
    frame_image
        .write_to(&mut buf, image::ImageOutputFormat::Jpeg(100))
        .map_err(|error| {
            event!(
                Level::ERROR,
                "Error while writing {:?} as encoded JPEG to buffer: {:?}",
                &path,
                error
            );
            FileError::Malformed(path.to_path_buf())
        })?;
    let data = buf.into_inner();
    Ok(data)
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
