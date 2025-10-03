pub use crate::prelude::*;

/// The status of the coverage range
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CoverageStatus {
    #[serde(rename = "at-edge")]
    AtEdge,
    #[serde(rename = "import-pending")]
    ImportPending,
    #[serde(rename = "imported")]
    Imported,
}
impl fmt::Display for CoverageStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AtEdge => "at-edge",
            Self::ImportPending => "import-pending",
            Self::Imported => "imported",
        };
        write!(f, "{}", s)
    }
}
