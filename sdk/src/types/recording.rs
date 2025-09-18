use crate::device_summary::DeviceSummary;
use crate::recording_import_status::RecordingImportStatus;
use crate::recording_metadata::RecordingMetadata;
use crate::site_summary::SiteSummary;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Recording {
    pub id: String,
    pub path: String,
    pub size: f32,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "importedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_at: Option<chrono::DateTime<chrono::Utc>>,
    pub start: String,
    pub end: String,
    #[serde(rename = "importStatus")]
    pub import_status: RecordingImportStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<SiteSummary>,
    #[serde(rename = "edgeSite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_site: Option<SiteSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<RecordingMetadata>>,
}
