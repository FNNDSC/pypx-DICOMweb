//! OpenAPI router definition for DICOMweb (QIDO, WADO-rs) routes.

use crate::models::*;
use crate::pypx_reader::{FailedJsonRead, PypxBaseNotADir, PypxReader};
use poem::web::Path;
use poem_openapi::param::Query;
use poem_openapi::OpenApi;
use std::path::PathBuf;

pub(crate) struct PypxDicomWebRouter {
    pypx: PypxReader,
}

#[OpenApi(prefix_path = "/dicomweb")]
impl PypxDicomWebRouter {
    pub fn new(base: PathBuf) -> Result<Self, PypxBaseNotADir> {
        PypxReader::new(base).map(|p| Self { pypx: p })
    }

    // TODO limit, offset, fuzzymatching, includefield, 00100020
    /// Query for studies.
    #[oai(path = "/studies", method = "get")]
    pub async fn query_studies(&self, StudyInstanceUID: Query<Option<String>>) -> QueryResponse {
        let study_instance_uid = StudyInstanceUID.0.as_ref().map(|s| s.as_str());
        let result = self.pypx.get_studies(study_instance_uid).await;
        QueryResponse::from(result)
    }

    #[oai(path = "/studies/:StudyInstanceUID/series", method = "get")]
    pub async fn series(&self, StudyInstanceUID: Path<String>) -> QueryResponse {
        let study_instance_uid = StudyInstanceUID.0.as_str();
        let result = self
            .pypx
            .get_series(study_instance_uid)
            .await
            .map_err(|_e| FailedJsonRead);
        QueryResponse::from(result)
    }
}
