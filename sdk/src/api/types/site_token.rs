use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A SiteToken resource provides site controllers with authentication to the api.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SiteToken {
    /// ID of the site token
    pub id: String,
    /// Timestamp of the site token's creation
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// ID of the associated Primary or Edge Site
    #[serde(rename = "siteId")]
    pub site_id: String,
}
