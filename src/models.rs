//! OpenAPI response model definitions for the DICOMweb API.
//!
//!
use crate::pypx_reader::FailedJsonRead;
use poem_openapi::{payload, types::Any, ApiResponse};
use serde_json::Value;

/// A response
#[derive(ApiResponse)]
pub enum QueryResponse {
    /// An OK response. The body contains a JSON list of DICOM metadata.
    /// N.B.: QIDO returns top-level JSON lists, however list is not a
    /// valid OpenAPI type.
    #[oai(status = 200)]
    Ok(payload::Json<Any<Vec<Value>>>),
    /// Internal server error
    #[oai(status = 500)]
    FailedJsonRead,
}

impl From<Result<Vec<Value>, FailedJsonRead>> for QueryResponse {
    fn from(value: Result<Vec<Value>, FailedJsonRead>) -> Self {
        match value {
            Ok(v) => Self::Ok(payload::Json(Any(v))),
            Err(_e) => Self::FailedJsonRead,
        }
    }
}
