use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteRecordingsKeyOrIdResponse {
    /// The recording ID
    #[serde(rename = "recordingId")]
    pub recording_id: String,
}
