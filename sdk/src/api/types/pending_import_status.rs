use serde::{Deserialize, Serialize};
use std::fmt;

/// Status of initiated import
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PendingImportStatus {
    #[serde(rename = "received")]
    Received,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "ready for indexing")]
    ReadyForIndexing,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "complete")]
    Complete,
}
impl fmt::Display for PendingImportStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Received => "received",
            Self::Processing => "processing",
            Self::ReadyForIndexing => "ready for indexing",
            Self::Error => "error",
            Self::Complete => "complete",
        };
        write!(f, "{}", s)
    }
}
