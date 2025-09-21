use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetRecordingsRequestSortBy {
    #[serde(rename = "deviceName")]
    DeviceName,
    #[serde(rename = "createdAt")]
    CreatedAt,
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "duration")]
    Duration,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "importedAt")]
    ImportedAt,
}
impl fmt::Display for GetRecordingsRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DeviceName => "deviceName",
            Self::CreatedAt => "createdAt",
            Self::Start => "start",
            Self::End => "end",
            Self::Duration => "duration",
            Self::Path => "path",
            Self::ImportedAt => "importedAt",
        };
        write!(f, "{}", s)
    }
}
