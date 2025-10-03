pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostSitesRequest {
    pub name: String,
    pub r#type: PostSitesRequestType,
    #[serde(rename = "retainRecordingsSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_recordings_seconds: Option<f64>,
}
