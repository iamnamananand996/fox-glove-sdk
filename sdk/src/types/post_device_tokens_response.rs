use crate::device_token::DeviceToken;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostDeviceTokensResponse {
    #[serde(flatten)]
    pub device_token_fields: DeviceToken,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}