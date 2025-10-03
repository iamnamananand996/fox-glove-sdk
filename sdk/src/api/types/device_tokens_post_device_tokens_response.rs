pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostDeviceTokensResponse {
    #[serde(flatten)]
    pub device_token_fields: DeviceToken,
    /// Generated token. This is only available on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
