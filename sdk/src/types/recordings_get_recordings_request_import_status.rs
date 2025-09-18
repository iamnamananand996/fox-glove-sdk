use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetRecordingsRequestImportStatus {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "importing")]
    Importing,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "complete")]
    Complete,
}
impl fmt::Display for GetRecordingsRequestImportStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::None => "none",
            Self::Pending => "pending",
            Self::Importing => "importing",
            Self::Failed => "failed",
            Self::Complete => "complete",
        };
        write!(f, "{}", s)
    }
}
