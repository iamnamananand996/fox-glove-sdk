use crate::pending_import_status::PendingImportStatus;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PendingImport {
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub filename: String,
    #[serde(rename = "importId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "pipelineStage")]
    pub pipeline_stage: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "siteId")]
    pub site_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PendingImportStatus>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "quarantinedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quarantined_at: Option<chrono::DateTime<chrono::Utc>>,
}
