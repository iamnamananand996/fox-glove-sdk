use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteMultipleImportsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
