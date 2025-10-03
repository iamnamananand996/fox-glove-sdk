pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Device {
    /// Opaque identifier
    pub id: String,
    /// Organization-chosen device name
    pub name: String,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    /// The retention period for recordings created on this device. If set to
    /// zero, recordings are retained indefinitely. This is only relevant for
    /// devices that have an agent installed.
    #[serde(rename = "retainRecordingsSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_recordings_seconds: Option<f64>,
    /// A key-value map, where each key is one of your pre-defined device custom properties.
    /// Keys which are not recognized as custom properties will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, DevicePropertiesValue>>,
}
