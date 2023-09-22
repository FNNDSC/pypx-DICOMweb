use std::path::{Path, PathBuf};
use pypx::StudyDataMeta;
use dicom::dictionary_std::tags;
use serde::Deserialize;
use serde_json::{json, Value};
use crate::pypx_deserializer::{LoadJsonFileError, read_study_data_meta};
use crate::translate::study_meta_to_dicomweb;

pub struct PypxReader {
    base: PathBuf,
    study_data_dir: PathBuf,
}

#[derive(thiserror::Error, Debug)]
#[error("Not a directory: {0:?}")]
pub struct PypxBaseNotADir(pub PathBuf);


impl PypxReader {

    pub fn new(base: PathBuf) -> Result<Self, PypxBaseNotADir> {
        let study_data_dir = base.join("log/studyData");

        if ![&base, &study_data_dir].iter().all(|p| p.is_dir()) {
            Err(PypxBaseNotADir(study_data_dir))
        } else {
            Ok(Self { base, study_data_dir })
        }
    }

    pub async fn get_studies(&self, study_instance_uid: Option<&str>) -> Result<Vec<Value>, FailedJsonRead> {
        if let Some(study_instance_uid) = study_instance_uid {
            let file = self.study_meta_file_for(study_instance_uid);
            match read_study_data_meta(&file).await {
                Ok(study_data_meta) => {
                    let data = study_meta_to_dicomweb(&study_data_meta);
                    Ok(vec![data])
                }
                Err(e) => {
                    match e {
                        LoadJsonFileError::NotFound => { Ok(vec![]) }
                        LoadJsonFileError::Failed => { Err(FailedJsonRead) }
                    }
                }
            }
        } else {
            // For now, we are not going to return anything. Just trying to get retrieve to work.
            // For later, OHIF doesn't care that we return everything all the time
            Ok(vec![])
        }
    }

    fn study_meta_file_for(&self, study_instance_uid: &str) -> PathBuf {
        let fname = format!("{study_instance_uid}-meta.json");
        self.study_data_dir.join(fname)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to read JSON file.")]
pub struct FailedJsonRead;
