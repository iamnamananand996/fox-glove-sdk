use serde::{Deserialize, Serialize};
use std::fmt;

/// The result of the deletion attempt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeleteDataImportsResponseDeletionResultsItemResult {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "notFound")]
    NotFound,
}
impl fmt::Display for DeleteDataImportsResponseDeletionResultsItemResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Success => "success",
            Self::NotFound => "notFound",
        };
        write!(f, "{}", s)
    }
}
