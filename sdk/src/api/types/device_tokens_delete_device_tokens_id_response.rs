pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteDeviceTokensIdResponse {
    /// The device token ID
    pub id: String,
}
