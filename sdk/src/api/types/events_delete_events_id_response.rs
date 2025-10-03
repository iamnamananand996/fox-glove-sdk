pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteEventsIdResponse {
    /// ID of the deleted event
    pub id: String,
}
