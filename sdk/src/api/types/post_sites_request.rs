use crate::sites_post_sites_request_type::PostSitesRequestType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSitesRequest {
    pub name: String,
    pub r#type: PostSitesRequestType,
    #[serde(rename = "retainRecordingsSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_recordings_seconds: Option<f64>,
}
