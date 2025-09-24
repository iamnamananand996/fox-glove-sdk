use crate::coverage_status::CoverageStatus;
use crate::device_summary::DeviceSummary;
use crate::recording_import_status::RecordingImportStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A coverage range represents a time span for which Foxglove has data for a given device. Recordings without devices are omitted except when queried by recording ID or import ID.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Coverage {
    /// ID of device.
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// Device summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceSummary>,
    /// Start of this coverage
    pub start: DateTime<Utc>,
    /// End of this coverage
    pub end: DateTime<Utc>,
    /// The status of the coverage range
    pub status: CoverageStatus,
    #[serde(rename = "importStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<RecordingImportStatus>,
}
