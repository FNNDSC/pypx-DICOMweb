use std::path::PathBuf;
use crate::pypx_reader::{PypxBaseNotADir, PypxReader};
use poem_openapi::OpenApi;
use crate::models::ListStudiesResponse;

pub(crate) struct PypxDicomWebRouter {
    pypx: PypxReader
}

#[OpenApi]
impl PypxDicomWebRouter {

    pub fn new(base: PathBuf) -> Result<Self, PypxBaseNotADir> {
        PypxReader::new(base).map(|p| Self {pypx: p})
    }

    #[oai(path = "/studies", method = "get")]
    pub async fn list_studies(&self) -> ListStudiesResponse {
        let result = self.pypx
            .get_studies(Some("1.2.840.113845.11.1000000001785349915.20130308061609.6346698"))
            .await;
        ListStudiesResponse::from(result)
    }
}

