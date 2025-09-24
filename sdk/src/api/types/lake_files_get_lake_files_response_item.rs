use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetLakeFilesResponseItem {
    /// The full path of the file in the lake bucket
    pub path: String,
    /// The topic names associated with this file
    pub topics: Vec<String>,
}
