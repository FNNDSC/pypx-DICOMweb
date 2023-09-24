#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct StudyDataMeta<'a> {
    pub PatientID: Cow<'a, str>,
    pub StudyDescription: Cow<'a, str>,
    pub StudyDate: Cow<'a, str>,
    pub StudyInstanceUID: Cow<'a, str>,
    pub PerformedStationAETitle: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudyDataSeriesMeta<'a> {
    pub SeriesInstanceUID: Cow<'a, str>,
    pub SeriesBaseDir: Cow<'a, str>,
    pub DICOM: HashMap<String, ValueAndLabel<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueAndLabel<'a> {
    pub value: Cow<'a, str>,
    pub label: Cow<'a, str>,
}
