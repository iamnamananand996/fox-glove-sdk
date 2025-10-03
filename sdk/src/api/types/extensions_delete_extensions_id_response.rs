pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteExtensionsIdResponse {
    /// The ID of the deleted extension
    pub id: String,
}
