use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteDeviceTokensIdResponse {
    /// The device token ID
    pub id: String,
}
