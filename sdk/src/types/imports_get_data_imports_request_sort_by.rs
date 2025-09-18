use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetDataImportsRequestSortBy {
    #[serde(rename = "importId")]
    ImportId,
    #[serde(rename = "deviceId")]
    DeviceId,
    #[serde(rename = "importTime")]
    ImportTime,
    #[serde(rename = "dataStart")]
    DataStart,
    #[serde(rename = "dataEnd")]
    DataEnd,
}
impl fmt::Display for GetDataImportsRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ImportId => "importId",
            Self::DeviceId => "deviceId",
            Self::ImportTime => "importTime",
            Self::DataStart => "dataStart",
            Self::DataEnd => "dataEnd",
        };
        write!(f, "{}", s)
    }
}
