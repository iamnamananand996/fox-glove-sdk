pub use crate::prelude::*;

/// An attachment resource represents information about an MCAP attachment imported to Foxglove.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordingAttachment {
    /// ID of the attachment
    pub id: String,
    /// ID of the recording with which the attachment is associated
    #[serde(rename = "recordingId")]
    pub recording_id: String,
    /// ID of the Primary Site where the attachment data is stored
    #[serde(rename = "siteId")]
    pub site_id: String,
    /// Name field from the attachment record
    pub name: String,
    /// Media type from the attachment record
    #[serde(rename = "mediaType")]
    pub media_type: String,
    /// Log time field from the attachment record
    #[serde(rename = "logTime")]
    pub log_time: String,
    /// Create time field from the attachment record
    #[serde(rename = "createTime")]
    pub create_time: String,
    /// CRC field from the attachment record as a decimal number
    pub crc: f64,
    /// Size of the attachment in bytes
    pub size: i64,
    /// A hash of the attachment content (algorithm subject to change). Two attachments
    /// with the same fingerprint will have identical content.
    pub fingerprint: String,
    /// Path of attachment in lake storage. Only provided for [self-hosted Primary Sites](https://docs.foxglove.dev/docs/primary-sites/introduction/#self-hosted).
    #[serde(rename = "lakePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lake_path: Option<String>,
}
