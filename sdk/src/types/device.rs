use crate::device_properties_value::DevicePropertiesValue;
use chrono::{DateTime, NaiveDate, Utc};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Device {
    pub id: String,
    pub name: String,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "retainRecordingsSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_recordings_seconds: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, DevicePropertiesValue>>,
}