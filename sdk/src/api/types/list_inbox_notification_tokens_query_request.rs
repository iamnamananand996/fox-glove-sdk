pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListInboxNotificationTokensQueryRequest {
    #[serde(rename = "siteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}
