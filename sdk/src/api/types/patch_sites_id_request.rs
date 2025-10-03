pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PatchSitesIdRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "retainRecordingsSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_recordings_seconds: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
