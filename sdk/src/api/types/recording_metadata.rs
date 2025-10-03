pub use crate::prelude::*;

/// MCAP metadata pertaining to a recording.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordingMetadata {
    pub name: String,
    pub metadata: HashMap<String, String>,
}
