use serde::{Deserialize, Serialize};
use std::fmt;

/// Output file format.
/// * `bag1` - output a .bag file
/// * `mcap` - output a .mcap file
/// * `mcap0` - Deprecated. Use `mcap`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostDataStreamRequestOutputFormat {
    #[serde(rename = "bag1")]
    Bag1,
    #[serde(rename = "mcap")]
    Mcap,
    #[serde(rename = "mcap0")]
    Mcap0,
}
impl fmt::Display for PostDataStreamRequestOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Bag1 => "bag1",
            Self::Mcap => "mcap",
            Self::Mcap0 => "mcap0",
        };
        write!(f, "{}", s)
    }
}
