use crate::delete_data_imports_response_deletion_results_item::DeleteDataImportsResponseDeletionResultsItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteDataImportsResponse {
    #[serde(rename = "deletionResults")]
    pub deletion_results: Vec<DeleteDataImportsResponseDeletionResultsItem>,
}