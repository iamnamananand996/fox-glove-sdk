use crate::imports_delete_data_imports_response_deletion_results_item_result::DeleteDataImportsResponseDeletionResultsItemResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteDataImportsResponseDeletionResultsItem {
    #[serde(rename = "importId")]
    pub import_id: String,
    pub result: DeleteDataImportsResponseDeletionResultsItemResult,
}
