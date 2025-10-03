pub use crate::prelude::*;

/// Output compression format for chunks. Only valid if `outputFormat` is `mcap`.
/// * `""` - no compression
/// * `zstd` - zstd compression
/// * `lz4` - LZ4 compression (default)
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
