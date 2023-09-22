//! OpenAPI response model definitions for the DICOMweb API.
use poem_openapi::{ApiResponse, Object, payload};
use serde_json::Value;
use crate::pypx_reader::FailedJsonRead;

/// A list of studies with some DICOM metadata.
#[derive(Object)]
pub struct ListOfStudies {
    result: Vec<Value>
}

/// A response for [ListOfStudies].
#[derive(ApiResponse)]
pub enum ListStudiesResponse {
    /// OK response
    #[oai(status = 200)]
    Ok(payload::Json<ListOfStudies>),
    /// Internal server error
    #[oai(status = 500)]
    FailedJsonRead,
}

impl From<Result<Vec<Value>, FailedJsonRead>> for ListStudiesResponse {
    fn from(value: Result<Vec<Value>, FailedJsonRead>) -> Self {
        match value {
            Ok(v) => Self::Ok(payload::Json(ListOfStudies { result: v })),
            Err(_e) => Self::FailedJsonRead
        }
    }
}
