pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostDeviceTokensRequest {
    #[serde(rename = "deviceId")]
    pub device_id: String,
}
