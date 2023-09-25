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

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceData<'a> {
    pub PatientID: Cow<'a, str>,
    pub StudyInstanceUID: Cow<'a, str>,
    pub SeriesInstanceUID: Cow<'a, str>,
    pub SeriesDescription: Cow<'a, str>,
    pub SeriesNumber: MaybeU32<'a>,
    pub SeriesDate: Cow<'a, str>,
    pub Modality: Cow<'a, str>,
    pub outputFile: Cow<'a, str>,
    pub imageObj: HashMap<Cow<'a, str>, FileStat<'a>>,
}

/// File's stat metadata.
/// Not complete.
/// https://github.com/FNNDSC/pypx/blob/7619c15f4d2303d6d5ca7c255d81d06c7ab8682b/pypx/smdb.py#L1306-L1317
#[derive(Debug, Serialize, Deserialize)]
pub struct FileStat<'a> {
    /// Important! Checked by smdb.py to count how many files are packed so far.
    pub FSlocation: Cow<'a, str>,
}

/// Something that is maybe a [u32], but in case it's not valid, is a [str].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeU32<'a> {
    U32(u32),
    Str(Cow<'a, str>),
}
