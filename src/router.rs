//! Router definition for DICOMweb (QIDO, WADO-rs) routes.

use crate::pypx_reader::{FailedJsonRead, PypxReader};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Json, Router};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub fn get_router(pypx: PypxReader) -> Router {
    Router::new()
        .route("/studies", get(get_studies))
        .with_state(Arc::new(pypx))
}

async fn get_studies(
    State(pypx): State<Arc<PypxReader>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Value>>, FailedJsonRead> {
    pypx.query_studies(&params).await.map(Json)
}

impl IntoResponse for FailedJsonRead {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
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
