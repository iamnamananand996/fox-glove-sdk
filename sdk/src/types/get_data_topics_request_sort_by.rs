use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetDataTopicsRequestSortBy {
    #[serde(rename = "topic")]
    Topic,
    #[serde(rename = "version")]
    Version,
}
impl fmt::Display for GetDataTopicsRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Topic => "topic",
            Self::Version => "version",
        };
        write!(f, "{}", s)
    }
}
