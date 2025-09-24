use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLayoutsQueryRequest {
    #[serde(rename = "updatedSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_since: Option<DateTime<Utc>>,
    #[serde(rename = "includeData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_data: Option<bool>,
}
