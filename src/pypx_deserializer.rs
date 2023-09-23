use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum LoadJsonFileError {
    #[error("File not found")]
    NotFound,
    #[error("File content is malformed")]
    Malformed,
    #[error("Error reading file")]
    IO, // #[error(transparent)]
        // IO(std::io::Error),
}

/// Read and deserialize a JSON file which looks like this:
///
/// ```json
/// {
///     "uselessKey": {
///         "theObjectThatGetsReturned", "isThisOne"
///     }
/// }
/// ```
pub async fn read_1member_json_file<P: AsRef<Path>, T: DeserializeOwned>(
    p: P,
) -> Result<T, LoadJsonFileError> {
    let data: HashMap<String, T> = read_json_file(p.as_ref()).await?;
    for value in data.into_values() {
        return Ok(value);
    }
    // log::error!("Malformed StudyDataMeta file, nothing found: {:?}", p.as_ref());
    Err(LoadJsonFileError::Malformed)
}

/// Read and deserialize a (small) JSON file.
pub async fn read_json_file<P: AsRef<Path>, T: DeserializeOwned>(
    p: P,
) -> Result<T, LoadJsonFileError> {
    let data = tokio::fs::read(p).await?;
    let parsed = serde_json::from_slice(&data)?;
    Ok(parsed)
}

impl From<std::io::Error> for LoadJsonFileError {
    fn from(error: std::io::Error) -> Self {
        if matches!(error.kind(), std::io::ErrorKind::NotFound) {
            LoadJsonFileError::NotFound
        } else {
            LoadJsonFileError::IO
        }
    }
}

impl From<serde_json::Error> for LoadJsonFileError {
    fn from(_value: serde_json::Error) -> Self {
        LoadJsonFileError::Malformed
    }
}
