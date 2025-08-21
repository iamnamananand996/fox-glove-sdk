use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordingAttachment {
    pub id: String,
    #[serde(rename = "recordingId")]
    pub recording_id: String,
    #[serde(rename = "siteId")]
    pub site_id: String,
    pub name: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "logTime")]
    pub log_time: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    pub crc: f64,
    pub size: i32,
    pub fingerprint: String,
    #[serde(rename = "lakePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lake_path: Option<String>,
}