pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PatchDeviceTokensIdRequest {
    pub enabled: bool,
}
