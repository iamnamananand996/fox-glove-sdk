pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteDevicesNameOrIdResponse {
    /// The ID of the deleted device
    pub id: String,
}
