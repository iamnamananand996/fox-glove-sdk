pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostDevicesRequest {
    pub name: DeviceName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, PostDevicesRequestPropertiesValue>>,
}
