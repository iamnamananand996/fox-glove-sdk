use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A DeviceToken authenticates device agents to the API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeviceToken {
    pub id: String,
    /// Timestamp of the device token's creation
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// ID of the associated device
    #[serde(rename = "deviceId")]
    pub device_id: String,
    /// Whether this token is enabled or not
    pub enabled: bool,
}
