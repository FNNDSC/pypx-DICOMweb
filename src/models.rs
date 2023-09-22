use poem_openapi::{ApiResponse, Object, payload};
use serde_json::Value;
use crate::pypx_reader::FailedJsonRead;

#[derive(Object)]
pub struct ListOfStudies {
    result: Vec<Value>
}

#[derive(ApiResponse)]
pub enum ListStudiesResponse {
    #[oai(status = 200)]
    Ok(payload::Json<ListOfStudies>),
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
