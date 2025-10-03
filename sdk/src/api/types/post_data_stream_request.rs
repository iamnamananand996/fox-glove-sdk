pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostDataStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    #[serde(rename = "outputFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<PostDataStreamRequestOutputFormat>,
    #[serde(rename = "compressionFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_format: Option<PostDataStreamRequestCompressionFormat>,
    #[serde(rename = "includeAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_attachments: Option<bool>,
    #[serde(rename = "isHosted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hosted: Option<bool>,
    #[serde(rename = "replayPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_policy: Option<PostDataStreamRequestReplayPolicy>,
    #[serde(rename = "replayLookbackSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_lookback_seconds: Option<f64>,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(rename = "importId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "recordingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_id: Option<String>,
    #[serde(rename = "recordingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
