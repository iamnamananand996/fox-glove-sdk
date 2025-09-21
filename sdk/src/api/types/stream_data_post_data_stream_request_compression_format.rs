use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostDataStreamRequestCompressionFormat {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "zstd")]
    Zstd,
    #[serde(rename = "lz4")]
    Lz4,
}
impl fmt::Display for PostDataStreamRequestCompressionFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Empty => "",
            Self::Zstd => "zstd",
            Self::Lz4 => "lz4",
        };
        write!(f, "{}", s)
    }
}
