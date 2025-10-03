pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostRecordingsKeyOrIdImportResponse {
    /// Recording ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "importStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<RecordingImportStatus>,
}
