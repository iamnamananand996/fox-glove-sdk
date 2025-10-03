pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostSiteInboxNotificationTokensResponse {
    #[serde(flatten)]
    pub inbox_notification_token_fields: InboxNotificationToken,
    /// Generated token. This is only available on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
