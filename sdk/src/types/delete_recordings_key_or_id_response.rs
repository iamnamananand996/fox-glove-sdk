use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteRecordingsKeyOrIdResponse {
    #[serde(rename = "recordingId")]
    pub recording_id: String,
}