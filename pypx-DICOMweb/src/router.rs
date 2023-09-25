//! Router definition for DICOMweb (QIDO, WADO-rs) routes.

use crate::errors::{FileError, ReadDirError};
use crate::pypx_reader::PypxReader;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Json, Router};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub fn get_router(pypx: PypxReader) -> Router {
    Router::new()
        .route("/studies", get(get_studies))
        .route("/studies/:study_instance_uid/series", get(get_series))
        .route(
            "/studies/:study_instance_uid/series/:series_instance_uid/metadata",
            get(get_series_metadata),
        )
        .with_state(Arc::new(pypx))
}

async fn get_studies(
    State(pypx): State<Arc<PypxReader>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Value>>, FileError> {
    pypx.query_studies(&params).await.map(Json)
}

async fn get_series(
    State(pypx): State<Arc<PypxReader>>,
    Path(study_instance_uid): Path<String>,
) -> Result<Json<Vec<Value>>, ReadDirError> {
    pypx.get_series(&study_instance_uid).await.map(Json)
}

async fn get_series_metadata(
    State(pypx): State<Arc<PypxReader>>,
    Path((study_instance_uid, series_instance_uid)): Path<(String, String)>,
) -> Result<Json<Vec<Value>>, FileError> {
    pypx.get_series_dicomweb_metadata(&study_instance_uid, &series_instance_uid)
        .await
        .map(Json)
}

impl IntoResponse for FileError {
    fn into_response(self) -> Response {
        let status = match &self {
            FileError::NotFound(_) => StatusCode::NOT_FOUND,
            FileError::Malformed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            FileError::IO(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, format!("{self:?}")).into_response()
    }
}

impl IntoResponse for ReadDirError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{self:?}")).into_response()
    }
}
