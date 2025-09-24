use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteEventsIdResponse {
    /// ID of the deleted event
    pub id: String,
}
