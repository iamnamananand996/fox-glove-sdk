use crate::site_token::SiteToken;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostSiteTokensResponse {
    #[serde(flatten)]
    pub site_token_fields: SiteToken,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
