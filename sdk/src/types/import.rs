use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Import {
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
    #[serde(rename = "importTime")]
    pub import_time: chrono::DateTime<chrono::Utc>,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "inputType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "outputType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
    #[serde(rename = "inputSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_size: Option<f32>,
    #[serde(rename = "totalOutputSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_output_size: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}