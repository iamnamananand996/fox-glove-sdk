use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MCAP metadata pertaining to a recording.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingMetadata {
    pub name: String,
    pub metadata: HashMap<String, String>,
}
