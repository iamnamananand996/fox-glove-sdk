use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostDataUploadResponse {
    pub link: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
}
