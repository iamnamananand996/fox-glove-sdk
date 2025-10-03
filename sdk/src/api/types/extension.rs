pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Extension {
    /// Assigned by backend
    pub id: String,
    /// Name as it appears in your package.json
    pub name: String,
    /// Publisher as it appears in your package.json
    pub publisher: String,
    /// Display name as it appears in your package.json
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Description as it appears in the active version's package.json
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Version string as it appears in the active version's package.json
    #[serde(rename = "activeVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_version: Option<String>,
    /// SHA-256 sum of the active version contents, encoded as hex
    #[serde(rename = "sha256Sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha_256_sum: Option<String>,
}
