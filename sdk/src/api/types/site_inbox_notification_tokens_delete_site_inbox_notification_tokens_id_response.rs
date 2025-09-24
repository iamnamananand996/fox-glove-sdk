use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteSiteInboxNotificationTokensIdResponse {
    /// ID of the deleted inbox notification token
    pub id: String,
}
