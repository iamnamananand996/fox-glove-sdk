use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Topic {
    pub encoding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "schemaEncoding")]
    pub schema_encoding: String,
    #[serde(rename = "schemaName")]
    pub schema_name: String,
    pub topic: String,
    pub version: String,
}