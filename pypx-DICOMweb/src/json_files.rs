//! Helper functions for reading JSON files.

use crate::errors::JsonFileError;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::path::Path;

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
) -> Result<T, JsonFileError> {
    let data: HashMap<String, T> = read_json_file(p.as_ref()).await?;
    data.into_values()
        .next()
        .ok_or_else(|| JsonFileError::Malformed(p.as_ref().to_path_buf()))
}

/// Read and deserialize a (small) JSON file.
pub async fn read_json_file<P: AsRef<Path>, T: DeserializeOwned>(p: P) -> Result<T, JsonFileError> {
    let data = tokio::fs::read(p.as_ref())
        .await
        .map_err(|e| JsonFileError::from_io_error(p.as_ref().to_path_buf(), e))?;
    let parsed = serde_json::from_slice(&data)
        .map_err(|_e| JsonFileError::Malformed(p.as_ref().to_path_buf()))?;
    Ok(parsed)
}
