pub use crate::prelude::*;

/// An inbox notification token identifies a site to the inbox-notifications endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InboxNotificationToken {
    pub id: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// Timestamp of the inbox notification token's creation
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// ID of the associated self-hosted Primary Site
    #[serde(rename = "siteId")]
    pub site_id: String,
}
