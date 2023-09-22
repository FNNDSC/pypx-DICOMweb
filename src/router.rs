use std::path::PathBuf;
use crate::pypx_reader::{PypxBaseNotADir, PypxReader};
use poem_openapi::OpenApi;
use poem_openapi::param::Query;
use crate::models::ListStudiesResponse;

pub(crate) struct PypxDicomWebRouter {
    pypx: PypxReader
}

#[OpenApi]
impl PypxDicomWebRouter {

    pub fn new(base: PathBuf) -> Result<Self, PypxBaseNotADir> {
        PypxReader::new(base).map(|p| Self {pypx: p})
    }

    // TODO limit, offset, fuzzymatching, includefield
    #[oai(path = "/studies", method = "get")]
    pub async fn list_studies(&self, StudyInstanceUID: Query<Option<String>>) -> ListStudiesResponse {
        let study_instance_uid = StudyInstanceUID.0.as_ref().map(|s| s.as_str());
        let result = self.pypx.get_studies(study_instance_uid).await;
        ListStudiesResponse::from(result)
    }
}

