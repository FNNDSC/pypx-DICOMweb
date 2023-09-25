//! Router definition for DICOMweb (QIDO, WADO-rs) routes.

use crate::constants::MULTIPART_BOUNDARY;
use crate::dicom::encode_frame;
use crate::errors::{FileError, ReadDirError};
use crate::pypx_reader::PypxReader;
use axum::extract::{Path, Query, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Json, Router};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{event, Level};

pub fn get_router(pypx: PypxReader) -> Router {
    Router::new()
        .route("/studies", get(get_studies))
        .route("/studies/:study_instance_uid/series", get(get_series))
        .route(
            "/studies/:study_instance_uid/series/:series_instance_uid/metadata",
            get(get_series_metadata),
        )
        .route("/studies/:study_instance_uid/series/:series_instance_uid/instances/:sop_instance_uid/frames/:frame", get(get_frame))
        .with_state(Arc::new(pypx))
}

async fn get_studies(
    State(pypx): State<Arc<PypxReader>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Value>>, QueryError> {
    let limit = if let Some(l) = params.get("limit") {
        l.parse().or(Err(QueryError::BadRequest(
            "limit must be a natural number",
        )))?
    } else {
        usize::MAX
    };
    pypx.query_studies(&params, limit)
        .await
        .map(Json)
        .map_err(|e| e.into())
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
    // TODO caching headers
    pypx.get_series_dicomweb_metadata(&study_instance_uid, &series_instance_uid)
        .await
        .map(Json)
}

/// Respond with a frame of a DICOM file encoded as JPEG wrapped with multipart.
/// N.B.: tightly coupled to implementation details of OHIF and friends.
async fn get_frame(
    State(pypx): State<Arc<PypxReader>>,
    Path((_study_instance_uid, series_instance_uid, sop_instance_uid, frame)): Path<(
        String,
        String,
        String,
        u32,
    )>,
) -> Result<impl IntoResponse, FileError> {
    let path = pypx
        .get_instance_fslocation(&series_instance_uid, &sop_instance_uid)
        .await?;
    let frame_data = encode_frame(path, frame - 1).await?;

    // I don't know what UID to use, but here's a list of UIDs which OHIF accpets:
    // https://github.com/OHIF/Viewers/blob/10ca35d5f497021abd562d457d11818474d02868/platform/core/src/utils/generateAcceptHeader.ts#L39-L55
    let uid = dicom::dictionary_std::uids::IMPLICIT_VR_LITTLE_ENDIAN;

    // transfer-syntax:
    // https://github.com/RadicalImaging/Static-DICOMWeb/blob/fb045851476facb24143eea7f97b763438059360/packages/static-wado-creator/lib/writer/ImageFrameWriter.js#L27
    // Content-Type deliminiter is "\r\n\r\n":
    // https://github.com/cornerstonejs/cornerstone3D/blob/d0d2fac80581648681521e4ddb6a6d9aad2087f9/packages/dicomImageLoader/src/imageLoader/wadors/getPixelData.ts#L64
    let content_type =
        format!("Content-Type: application/octet-stream;transfer-syntax={uid}\r\n\r\n");

    let headers = [
        (header::ETAG, format!("\"{}/{}\"", sop_instance_uid, frame)),
        (header::CONTENT_TYPE, "multipart/related".to_string()),
    ];

    let size_estimate = MULTIPART_BOUNDARY.len()
        + content_type.len()
        + frame_data.len()
        + MULTIPART_BOUNDARY.len()
        + 64;
    let mut body: Vec<u8> = Vec::with_capacity(size_estimate);

    // boundary is separated by "\r\n":
    // https://github.com/cornerstonejs/cornerstone3D/blob/d0d2fac80581648681521e4ddb6a6d9aad2087f9/packages/dicomImageLoader/src/imageLoader/wadors/getPixelData.ts#L71
    body.extend(MULTIPART_BOUNDARY);
    body.extend(b"\r\n");
    body.extend(content_type.as_bytes());
    body.extend(frame_data);
    body.extend(b"\r\n");
    body.extend(MULTIPART_BOUNDARY);
    body.extend(b"--");

    let response = (headers, body).into_response();
    Ok(response)
}

#[derive(thiserror::Error, Debug)]
enum QueryError {
    #[error("{0}")]
    BadRequest(&'static str),
    #[error(transparent)]
    Err(#[from] FileError),
}

impl IntoResponse for QueryError {
    fn into_response(self) -> Response {
        match self {
            QueryError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            QueryError::Err(e) => e.into_response(),
        }
    }
}

impl IntoResponse for FileError {
    fn into_response(self) -> Response {
        let status = match &self {
            FileError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => {
                event!(Level::ERROR, "{:?}", self);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        (status, format!("{self:?}")).into_response()
    }
}

impl IntoResponse for ReadDirError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{self:?}")).into_response()
    }
}
