pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostDataUploadResponse {
    /// A signed upload URL. Upload your data to this URL using a PUT request.
    pub link: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
}
