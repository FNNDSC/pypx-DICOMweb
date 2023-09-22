use std::path::PathBuf;
use pypx::StudyDataMeta;
use dicom::dictionary_std::tags;
use serde_json::{json, Value};

pub struct PypxDb {
    base: PathBuf,
    study_data_dir: PathBuf,
}

#[derive(thiserror::Error, Debug)]
#[error("Not a directory: {0:?}")]
pub struct PypxBaseNotADir(pub PathBuf);


impl PypxDb {

    pub fn new(base: PathBuf) -> Result<Self, PypxBaseNotADir> {
        let study_data_dir = base.join("log/studyData");

        if ![&base, &study_data_dir].iter().all(|p| p.is_dir()) {
            Err(PypxBaseNotADir(study_data_dir))
        } else {
            Ok(Self { base, study_data_dir })
        }
    }

    pub async fn get_studies(&self, study_instance_uid: Option<&str>) -> Vec<serde_json::Value> {
        if let Some(study_instance_uid) = study_instance_uid {
            vec![]
        } else {
            // OHIF doesn't care that we return everything all the time
            vec![]
        }
    }
}

fn study_meta_to_dicomweb(data: &StudyDataMeta) -> Value {
    json!({
        tag2str(tags::PATIENT_ID): {
            "vr": "LO",
            "Value": [
                &data.PatientID
            ]
        },
        tag2str(tags::STUDY_DESCRIPTION): {
            "vr": "LO",
            "Value": [
                &data.StudyDescription
            ]
        },
        tag2str(tags::STUDY_DATE): {
            "vr": "DA",
            "Value": [
                &data.StudyDate
            ]
        },
        tag2str(tags::STUDY_INSTANCE_UID): {
            "vr": "UI",
            "Value": [
                &data.StudyInstanceUID
            ]
        },
        tag2str(tags::PERFORMED_STATION_AE_TITLE): {
            "vr": "AE",
            "Value": [
                &data.PerformedStationAETitle
            ]
        }
    })
}

fn tag2str(tag: dicom::core::Tag) -> String {
    format!("{:04X}{:04X}", tag.0, tag.1)
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;
    use super::*;
    use pypx::StudyDataMeta;
    use rstest::*;

    #[rstest]
    fn test_it_works(example_study_meta: StudyDataMeta) {
        let thing = study_meta_to_dicomweb(&example_study_meta);
        dbg!(thing);
    }

    #[fixture]
    fn example_study_meta() -> StudyDataMeta<'static> {
        StudyDataMeta {
            PatientID: Cow::from("1449c1d"),
            StudyDescription: Cow::from("MR-Brain w/o Contrast"),
            StudyDate: Cow::from("20130308"),
            StudyInstanceUID: Cow::from("1.2.840.113845.11.1000000001785349915.20130308061609.6346698"),
            PerformedStationAETitle: Cow::from("Not defined")
        }
    }
}
