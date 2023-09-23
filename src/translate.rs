//! Helper functions related to translating to DICOMweb response schemas.

use dicom::dictionary_std::tags;
use pypx::*;
use serde_json::{json, Value};

pub fn study_meta_to_dicomweb(data: &StudyDataMeta) -> Value {
    json!({
        tag2str(tags::PATIENT_ID): {
            "vr": "LO",
            "Value": [
                data.PatientID
            ]
        },
        tag2str(tags::STUDY_DESCRIPTION): {
            "vr": "LO",
            "Value": [
                data.StudyDescription
            ]
        },
        tag2str(tags::STUDY_DATE): {
            "vr": "DA",
            "Value": [
                data.StudyDate
            ]
        },
        tag2str(tags::STUDY_INSTANCE_UID): {
            "vr": "UI",
            "Value": [
                data.StudyInstanceUID
            ]
        },
        tag2str(tags::PERFORMED_STATION_AE_TITLE): {
            "vr": "AE",
            "Value": [
                data.PerformedStationAETitle
            ]
        }
    })
}

pub fn series_meta_to_dicomweb(data: &StudyDataSeriesMeta, num_instances: usize) -> Value {
    json!({
        // missing SpecificCharacterSet, CS
        tag2str(tags::SERIES_DATE): {
            "vr": "DA",
            "Value": [ get_pypxed_tag(data, "SeriesDate") ]
        },
        tag2str(tags::SERIES_TIME): {
            "vr": "TM",
            "Value": [ get_pypxed_tag(data, "SeriesTime") ]
        },
        tag2str(tags::MODALITY): {
            "vr": "MR",
            "Value": [ get_pypxed_tag(data, "Modality") ]
        },
        // missing Manufacturer, LO
        // missing ReferringPhysicianName, PN
        tag2str(tags::SERIES_DESCRIPTION): {
            "vr": "LO",
            "Value": [ get_pypxed_tag(data, "SeriesDescription") ]
        },
        // missing ManufacturerModelName, LO
        // missing PrivateCreator, CS
        // missing (0009,1011)
        // missing (0009,1012)
        // missing BodyPartExamined, CS
        // missing ProtocolName, LO
        tag2str(tags::STUDY_INSTANCE_UID): {
            "vr": "UI",
            "Value": [ get_pypxed_tag(data, "StudyInstanceUID") ]
        },
        tag2str(tags::SERIES_INSTANCE_UID): {
            "vr": "UI",
            "Value": [ get_pypxed_tag(data, "SeriesInstanceUID") ]
        },
        tag2str(tags::SERIES_NUMBER): {
            "vr": "IS",
            "Value": [ try_parse_int(get_pypxed_tag(data, "SeriesNumber")) ]
        },
        tag2str(tags::NUMBER_OF_SERIES_RELATED_INSTANCES): {
            "vr": "IS",
            "Value": [ num_instances ]
        }
    })
}

fn get_pypxed_tag<'a>(data: &'a StudyDataSeriesMeta, tag_name: &str) -> &'a str {
    data.DICOM
        .get(tag_name)
        .map(|p| p.value.as_ref())
        .unwrap_or_default()
}

fn tag2str(tag: dicom::core::Tag) -> String {
    format!("{:04X}{:04X}", tag.0, tag.1)
}


fn try_parse_int(num: &str) -> Value {
    num.parse::<i32>().map(|n| json!(n)).unwrap_or_else(|_e| json!(num))
}

#[cfg(test)]
mod test {
    use super::*;
    use pypx::StudyDataMeta;
    use rstest::*;
    use std::borrow::Cow;

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
            StudyInstanceUID: Cow::from(
                "1.2.840.113845.11.1000000001785349915.20130308061609.6346698",
            ),
            PerformedStationAETitle: Cow::from("Not defined"),
        }
    }
}
