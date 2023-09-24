//! Router definition for DICOMweb (QIDO, WADO-rs) routes.

use crate::pypx_reader::{PypxReader};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Json, Router};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use crate::errors::JsonFileError;

pub fn get_router(pypx: PypxReader) -> Router {
    Router::new()
        .route("/studies", get(get_studies))
        .route("/studies/:study_instance_uid/series", get(get_series))
        .with_state(Arc::new(pypx))
}

async fn get_studies(
    State(pypx): State<Arc<PypxReader>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Value>>, JsonFileError> {
    pypx.query_studies(&params).await.map(Json)
}

async fn get_series(
    State(pypx): State<Arc<PypxReader>>,
    Path(study_instance_uid): Path<String>
) -> Result<Json<Vec<Value>>, JsonFileError> {
    // pypx.get_series(&study_instance_uid).await.map(Json)
    todo!()
}

impl IntoResponse for JsonFileError {
    fn into_response(self) -> Response {
        let status = match &self {
            JsonFileError::NotFound(_) => StatusCode::NOT_FOUND,
            JsonFileError::Malformed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            JsonFileError::IO(_, _) => StatusCode::INTERNAL_SERVER_ERROR
        };
        (status, format!("{self:?}")).into_response()
    }
}

//
// #[OpenApi(prefix_path = "/dicomweb")]
// impl PypxDicomWebRouter {
//     pub fn new(base: PathBuf) -> Result<Self, PypxBaseNotADir> {
//         PypxReader::new(base).map(|p| Self { pypx: p })
//     }
//
//     // TODO limit, offset, fuzzymatching, includefield, 00100020
//     /// Query for studies.
//     #[oai(path = "/studies", method = "get")]
//     pub async fn query_studies(&self, StudyInstanceUID: Query<Option<String>>) -> QueryResponse {
//         let study_instance_uid = StudyInstanceUID.0.as_ref().map(|s| s.as_str());
//         let result = self.pypx.get_studies(study_instance_uid).await;
//         QueryResponse::from(result)
//     }
//
//     #[oai(path = "/studies/:StudyInstanceUID/series", method = "get")]
//     pub async fn series(&self, StudyInstanceUID: Path<String>) -> QueryResponse {
//         let study_instance_uid = StudyInstanceUID.0.as_str();
//         let result = self
//             .pypx
//             .get_series(study_instance_uid)
//             .await
//             .map_err(|_e| FailedJsonRead);
//         QueryResponse::from(result)
//     }
// }
