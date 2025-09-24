use serde::{Deserialize, Serialize};

/// A topic to which messages can be published.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Topic {
    /// Encoding of the topic
    pub encoding: String,
    /// Full schema, base-64 encoded; included only if the request's includeSchemas is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Encoding of the topic schema
    #[serde(rename = "schemaEncoding")]
    pub schema_encoding: String,
    /// Name of the topic schema
    #[serde(rename = "schemaName")]
    pub schema_name: String,
    /// Topic name
    pub topic: String,
    /// Topic version
    pub version: String,
}
