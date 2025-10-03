pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostDataStreamResponse {
    /// A signed url to access the data. This link expires after 15 seconds.
    pub link: String,
}
