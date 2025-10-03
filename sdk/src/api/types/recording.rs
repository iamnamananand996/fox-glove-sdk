pub use crate::prelude::*;

/// A resource representing the content of an MCAP file or ROS bag managed by Foxglove.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Recording {
    /// ID of the recording
    pub id: String,
    /// The filepath of the recording as it was originally provided to Foxglove.
    /// For recordings originated at the edge, this is the full path of the recording file in
    /// the edge controller store. For recordings uploaded directly to Foxglove, this is
    /// the `filename` query arg provided to `/data/upload`.
    pub path: String,
    /// The size of the recording file, in bytes
    pub size: f64,
    /// Timestamp when the recording file was added to Foxglove.
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// Timestamp when the recording was imported to a primary site.
    #[serde(rename = "importedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_at: Option<DateTime<Utc>>,
    /// The log time of the first message in the recording.
    pub start: String,
    /// The log time of the last message in the recording.
    pub end: String,
    #[serde(rename = "importStatus")]
    pub import_status: RecordingImportStatus,
    /// The primary site for this recording.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<SiteSummary>,
    /// The edge site for this recording.
    #[serde(rename = "edgeSite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_site: Option<SiteSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceSummary>,
    /// The unique, user-specified key assigned to a recording in MCAP metadata or during the upload
    /// process. This can be used to identify the recording, in addition to its `id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The metadata records of the original MCAP file content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<RecordingMetadata>>,
}
