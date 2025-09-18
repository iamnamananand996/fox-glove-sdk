use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Extension {
    pub id: String,
    pub name: String,
    pub publisher: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "activeVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_version: Option<String>,
    #[serde(rename = "sha256Sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_256_sum: Option<String>,
}
