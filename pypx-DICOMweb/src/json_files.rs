//! Helper functions for reading JSON files.

use crate::errors::FileError;
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
) -> Result<T, FileError> {
    let data: HashMap<String, T> = read_json_file(p.as_ref()).await?;
    data.into_values().next().ok_or_else(|| {
        FileError::Malformed(p.as_ref().to_path_buf(), "Empty object".to_string(), None)
    })
}

/// Read and deserialize a (small) JSON file.
pub async fn read_json_file<P: AsRef<Path>, T: DeserializeOwned>(p: P) -> Result<T, FileError> {
    let data = tokio::fs::read(p.as_ref())
        .await
        .map_err(|e| FileError::from_io_error(p.as_ref().to_path_buf(), e))?;
    let parsed = serde_json::from_slice(&data).map_err(|error| {
        FileError::Malformed(
            p.as_ref().to_path_buf(),
            "Could not deserialize".to_string(),
            Some(error.into()),
        )
    })?;
    Ok(parsed)
}
