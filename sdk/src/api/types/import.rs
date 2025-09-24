use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    /// Opaque ID
    pub id: String,
    #[serde(rename = "importId")]
    pub import_id: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    pub filename: String,
    /// Imported timestamp
    #[serde(rename = "importTime")]
    pub import_time: DateTime<Utc>,
    /// Message log time start of imported data
    pub start: DateTime<Utc>,
    /// Message log time end of imported data
    pub end: DateTime<Utc>,
    /// Supported input type: ROS 1 bag ("bag1") or MCAP v0.x ("mcap0") file
    #[serde(rename = "inputType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    /// Supported output type: ROS 1 bag ("bag1") or MCAP v0.x ("mcap0") file
    #[serde(rename = "outputType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
    /// Size in bytes of the uploaded file
    #[serde(rename = "inputSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_size: Option<f32>,
    /// Size in bytes of the data after processing
    #[serde(rename = "totalOutputSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_output_size: Option<f32>,
    /// Message log time start date in YYYYMMDD format e.g. 20170322
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}
