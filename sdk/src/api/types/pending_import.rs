pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PendingImport {
    /// When the pending import was created
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// ID of device
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// Name of the device from which the import originated
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// Error message, if status is "error"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Filename of the import
    pub filename: String,
    /// ID of import; undefined until status is "complete"
    #[serde(rename = "importId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    /// ID of org to which the import belongs
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// stage in the import pipeline the import currently occupies
    #[serde(rename = "pipelineStage")]
    pub pipeline_stage: String,
    /// ID of the import request
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// ID of Primary Site where import will be stored
    #[serde(rename = "siteId")]
    pub site_id: String,
    /// Status of initiated import
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PendingImportStatus>,
    /// When the pending import was last updated
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    /// When the pending import was quarantined
    #[serde(rename = "quarantinedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quarantined_at: Option<DateTime<Utc>>,
}
