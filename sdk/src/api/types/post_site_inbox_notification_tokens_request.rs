use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostSiteInboxNotificationTokensRequest {
    #[serde(rename = "siteId")]
    pub site_id: String,
}
