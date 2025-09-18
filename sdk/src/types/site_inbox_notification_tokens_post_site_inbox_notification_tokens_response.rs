use crate::inbox_notification_token::InboxNotificationToken;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostSiteInboxNotificationTokensResponse {
    #[serde(flatten)]
    pub inbox_notification_token_fields: InboxNotificationToken,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
