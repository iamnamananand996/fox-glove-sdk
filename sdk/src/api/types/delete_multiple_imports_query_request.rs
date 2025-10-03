pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteMultipleImportsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
