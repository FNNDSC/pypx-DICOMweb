//! Reads data from a pypx-organized directory, presenting it in "DICOMweb format."

use crate::errors::{JsonFileError, PypxBaseNotADir, ReadDirError};
use crate::json_files::{read_1member_json_file, read_json_file};
use crate::translate::{series_meta_to_dicomweb, study_meta_to_dicomweb};
use futures::StreamExt;
use pypx::{StudyDataMeta, StudyDataSeriesMeta};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio_stream::wrappers::ReadDirStream;
use crate::constants;

/// Creates a closure suitable for use by [StreamExt::filter_map]
/// which filters paths by extension.
///
/// Using a macro is necessary because returning an async closure from a function
/// is currently unstable: https://github.com/rust-lang/rust/issues/62290
macro_rules! select_files_by_extension {
    ($extension:expr) => {
        |path| async move {
            if path.is_file() && path.file_name()?.to_str()?.ends_with($extension) {
                Some(path)
            } else {
                None
            }
        }
    };
}

/// Reader of a pypx-organized directory of DICOM and JSON files.
pub struct PypxReader {
    study_data_dir: PathBuf,
    series_data_dir: PathBuf,
}

impl PypxReader {
    /// Instantiate a [PypxReader], checking to make sure the right directories exist
    /// (`log/studyData`, `log/seriesData`).
    pub fn new(base: PathBuf) -> Result<Self, PypxBaseNotADir> {
        let log_dir = base.join("log");
        let study_data_dir = log_dir.join("studyData");
        let series_data_dir = log_dir.join("seriesData");

        let all = [&study_data_dir, &series_data_dir];
        if !all.iter().all(|p| p.is_dir()) {
            Err(PypxBaseNotADir(study_data_dir))
        } else {
            Ok(Self {
                study_data_dir,
                series_data_dir,
            })
        }
    }

    /// Find study metadata from the pypx-organized filesystem.
    /// Returns data in DICOMweb's response schema.
    pub async fn query_studies(
        &self,
        query: &HashMap<String, String>,
    ) -> Result<Vec<Value>, JsonFileError> {
        let studies = if let Some(study_instance_uid) = query.get("StudyInstanceUID") {
            flatten_notfound_error(self.get_study(study_instance_uid).await)?
        } else {
            self.ls_studies(query).await
        };
        let dicomweb_response = studies.iter().map(study_meta_to_dicomweb).collect();
        Ok(dicomweb_response)
    }

    /// Find studies matching a given filter, returning study metadata as a DICOMweb response.
    async fn ls_studies<'a>(&'a self, query: &'a HashMap<String, String>) -> Vec<StudyDataMeta> {
        let path = &self.study_data_dir;
        let read_dir = tokio::fs::read_dir(path)
            .await
            .map_err(|e| ReadDirError(path.to_path_buf(), e.kind()))
            // assuming study dir exists, we checked it in Self::new()
            .expect(&format!("{:?} is not a directory", &self.study_data_dir));
        // TODO limit, offset
        ReadDirStream::new(read_dir)
            .filter_map(report_then_discard_error)
            .map(|entry| entry.path())
            .filter_map(select_files_by_extension!("-meta.json"))
            .map(|path| read_json_file(path))
            .buffer_unordered(4)
            .filter_map(report_then_discard_error)
            .filter_map(|study| study_matches_wrapper(study, query) )
            .collect()
            .await
    }

    /// Get a single study and its metadata.
    async fn get_study(&self, study_instance_uid: &str) -> Result<StudyDataMeta, JsonFileError> {
        let file = self.study_meta_file_for(study_instance_uid);
        let result: Result<StudyDataMeta, _> = read_1member_json_file(file).await;
        result
    }

    pub async fn get_series(&self, study_instance_uid: &str) -> Result<Vec<Value>, ReadDirError> {
        let path = self.series_meta_dir_of(study_instance_uid);
        let read_dir = tokio::fs::read_dir(&path)
            .await
            .map_err(|e| ReadDirError(path, e.kind()))?;
        let datas = ReadDirStream::new(read_dir)
            .filter_map(report_then_discard_error)
            .map(|entry| entry.path())
            .filter_map(select_files_by_extension!("-meta.json"))
            .map(|path| self.get_series_data(path))
            .buffer_unordered(4)
            .filter_map(report_then_discard_error)
            .collect()
            .await;
        Ok(datas)
    }

    // Helper functions for getting information from files and directories
    // --------------------------------------------------------------------------------

    /// Given a path `log/studyData/XXX-series/X-meta.json`, produce the metadata of the
    /// corresponding series including `NumberOfSeriesRelatedInstances`.
    async fn get_series_data(&self, path: PathBuf) -> Result<Value, JsonFileError> {
        let data: StudyDataSeriesMeta = read_1member_json_file(&path).await?;
        let series_instance_uid = data.SeriesInstanceUID.as_ref();
        let num_instances = self.count_instances(series_instance_uid).await.unwrap_or(0);
        let value = series_meta_to_dicomweb(&data, num_instances);
        Ok::<Value, JsonFileError>(value)
    }

    /// Count the number of DICOM instances in the specified series.
    async fn count_instances(&self, series_instance_uid: &str) -> Result<usize, std::io::Error> {
        let path = self.instances_json_dir_for(series_instance_uid);
        let read_dir = tokio::fs::read_dir(path).await?;
        let count = ReadDirStream::new(read_dir)
            .filter_map(report_then_discard_error)
            .map(|entry| entry.path())
            .filter_map(select_files_by_extension!(".dcm.json"))
            .count()
            .await;
        Ok(count)
    }

    // Helper functions related to the pypx organization of files
    // --------------------------------------------------------------------------------

    fn study_meta_file_for(&self, study_instance_uid: &str) -> PathBuf {
        let name = format!("{study_instance_uid}-meta.json");
        self.study_data_dir.join(name)
    }

    fn series_meta_dir_of(&self, study_instance_uid: &str) -> PathBuf {
        let name = format!("{study_instance_uid}-series");
        self.study_data_dir.join(name)
    }

    fn instances_json_dir_for(&self, series_instance_uid: &str) -> PathBuf {
        let name = format!("{series_instance_uid}-img");
        self.series_data_dir.join(name)
    }
}

/// A wrapper for [study_matches] with lifetime annotations
/// so that it may be used with [StreamExt::filter_map].
async fn study_matches_wrapper<'a>(study: StudyDataMeta<'a>, query: &'a HashMap<String, String>) -> Option<StudyDataMeta<'a>> {
    if study_matches(&study, query) {
        Some(study)
    } else {
        None
    }
}

/// Returns `true` if the given study matches the given query.
fn study_matches(study: &StudyDataMeta, query: &HashMap<String, String>) -> bool {
    if let Some(patient_id) = query.get(constants::PATIENT_ID) {
        if patient_id != study.PatientID.as_ref() {
            return false;
        }
    }
    true
}

fn flatten_notfound_error<T>(result: Result<T, JsonFileError>) -> Result<Vec<T>, JsonFileError> {
    match result {
        Ok(value) => Ok(vec![value]),
        Err(error) => match error {
            JsonFileError::NotFound(_path) => Ok(vec![]),
            _ => Err(error),
        },
    }
}

async fn report_then_discard_error<T, E: std::error::Error>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(error) => {
            eprintln!("Error: {error:?}"); // TODO use actual logging
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dicom::core::DataDictionary;
    use dicom::object::StandardDataDictionary;

    use serde_json::json;

    #[test]
    fn just_print_out_what_i_need_for_each_series() {
        let data: Value = json!({
            "0020000D": {
                "vr": "UI",
                "Value": [
                    "1.3.6.1.4.1.14519.5.2.1.3023.4024.215308722288168917637555384485"
                ]
            },
            "0008103E": {
                "vr": "LO",
                "Value": [
                    "SAG T1 T-SPINE"
                ]
            },
            "00200011": {
                "vr": "IS",
                "Value": [
                    7
                ]
            },
            "0020000E": {
                "Value": [
                    "1.3.6.1.4.1.14519.5.2.1.3023.4024.332634904672834192826308613876"
                ]
            },
            "00080060": {
                "vr": "CS",
                "Value": [
                    "MR"
                ]
            },
            "00080021": {
                "vr": "DA",
                "Value": [
                    "20020315"
                ]
            },
            "00080031": {
                "vr": "TM",
                "Value": [
                    "170746"
                ]
            },
            "00080005": {
                "vr": "CS",
                "Value": [
                    "ISO_IR 100"
                ]
            },
            "00080070": {
                "vr": "LO",
                "Value": [
                    "GE MEDICAL SYSTEMS"
                ]
            },
            "00080090": {
                "vr": "PN"
            },
            "00081090": {
                "vr": "LO",
                "Value": [
                    "SIGNA EXCITE"
                ]
            },
            "00180015": {
                "vr": "CS",
                "Value": [
                    "TSPINE"
                ]
            },
            "00181030": {
                "vr": "LO",
                "Value": [
                    "8CH- THORACIC SPINE/7"
                ]
            },
            "00090010": {
                "Value": [
                    "dedupped"
                ],
                "vr": "CS"
            },
            "00091011": {
                "Value": [
                    "d27a7b6a5d0f63bf519c38640894b57a0febb2886ed1ed20b0b5caf9c8c77dd6"
                ]
            },
            "00091012": {
                "Value": [
                    "series"
                ]
            },
            "00201209": {
                "vr": "IS",
                "Value": [
                    24
                ]
            }
        });

        if let Value::Object(m) = data {
            for (k, v) in m {
                let tag: dicom::core::Tag = k.clone().parse().unwrap();
                let tag_name = StandardDataDictionary.by_tag(tag).map(|e| e.alias);
                println!("{}, {:?}, {}", tag, tag_name, v)
            }
        } else {
            panic!("not a map")
        }
    }
}
