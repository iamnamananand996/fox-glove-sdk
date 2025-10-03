pub use crate::prelude::*;

/// The import status of recordings. Status will be one of:
/// - `none`: The recording has not yet been imported, and import has not been requested.
/// - `pending`: Foxglove has received a request to import this recording.
/// - `importing`: The recording data is being processed for access via Foxglove.
/// - `failed`: The recording data could not be imported.
/// - `complete`: The contents of the recording are available for access via Foxglove.
///
/// Note: `none` and `pending` statuses are applicable only to recordings originating from an Edge Site or Foxglove Agent.
///
/// The set of `importStatus` values may expand in the future.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RecordingImportStatus {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "importing")]
    Importing,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "complete")]
    Complete,
}
impl fmt::Display for RecordingImportStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::None => "none",
            Self::Pending => "pending",
            Self::Importing => "importing",
            Self::Failed => "failed",
            Self::Complete => "complete",
        };
        write!(f, "{}", s)
    }
}
