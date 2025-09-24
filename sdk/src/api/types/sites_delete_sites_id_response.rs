use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteSitesIdResponse {
    /// ID of the deleted site
    pub id: String,
}
