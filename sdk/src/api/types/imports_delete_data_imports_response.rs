pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteDataImportsResponse {
    #[serde(rename = "deletionResults")]
    pub deletion_results: Vec<DeleteDataImportsResponseDeletionResultsItem>,
}
