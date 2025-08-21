use crate::device_summary::DeviceSummary;
use crate::coverage_status::CoverageStatus;
use crate::recording_import_status::RecordingImportStatus;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Coverage {
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceSummary>,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
    pub status: CoverageStatus,
    #[serde(rename = "importStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<RecordingImportStatus>,
}