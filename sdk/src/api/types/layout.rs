pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Layout {
    /// ID of the layout
    pub id: String,
    /// Name of the layout
    pub name: String,
    /// Timestamp when the layout was created on the server
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    /// Timestamp when the layout was last updated on the server
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    /// Timestamp when the layout was last saved locally
    #[serde(rename = "savedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_at: Option<DateTime<Utc>>,
    /// Name of the folder the layout belongs to.
    #[serde(rename = "folderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_name: Option<String>,
    pub permission: LayoutPermission,
    /// An object containing the layout data.
    ///
    /// Note: The layout data format is not considered stable and may change over time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
}
