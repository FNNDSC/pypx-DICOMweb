use std::collections::HashMap;
use std::path::Path;
use pypx::StudyDataMeta;
use serde::de::DeserializeOwned;

#[derive(thiserror::Error, Debug)]
pub enum LoadJsonFileError {
    #[error("File not found")]
    NotFound,
    #[error("Failed to load JSON data")]
    Failed
}

pub async fn read_study_data_meta<'a, P: AsRef<Path>>(p: P) -> Result<StudyDataMeta<'a>, LoadJsonFileError> {
    let data: HashMap<String, StudyDataMeta> = read_json_file(p.as_ref()).await?;
    for value in data.into_values() {
        return Ok(value);
    }
    // log::error!("Malformed StudyDataMeta file, nothing found: {:?}", p.as_ref());
    Err(LoadJsonFileError::Failed)
}

impl From<std::io::Error> for LoadJsonFileError {
    fn from(value: std::io::Error) -> Self {
        if matches!(value.kind(), std::io::ErrorKind::NotFound) {
            LoadJsonFileError::NotFound
        } else {
            LoadJsonFileError::Failed
        }
    }
}

impl From<serde_json::Error> for LoadJsonFileError {
    fn from(_value: serde_json::Error) -> Self {
        LoadJsonFileError::Failed
    }
}


async fn read_json_file<P: AsRef<Path>, T: DeserializeOwned>(p: P) -> Result<T, LoadJsonFileError> {
    let data = tokio::fs::read(p).await?;
    let parsed = serde_json::from_slice(&data)?;
    Ok(parsed)
}
