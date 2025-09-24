use crate::device_summary::DeviceSummary;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// ID of the event
    pub id: String,
    /// Event start time (inclusive)
    pub start: DateTime<Utc>,
    /// Event end time (inclusive)
    pub end: DateTime<Utc>,
    /// ID of the device associated with the event.
    ///
    /// **deprecated**: Use `device.id` instead.
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceSummary>,
    /// Any metadata associated with the event
    pub metadata: HashMap<String, serde_json::Value>,
    /// When the event was created
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// When the event was last updated
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
