pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteDataImportsResponseDeletionResultsItem {
    /// The import ID that was provided
    #[serde(rename = "importId")]
    pub import_id: String,
    /// The result of the deletion attempt.
    pub result: DeleteDataImportsResponseDeletionResultsItemResult,
}
