use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetLakeFilesResponseItem {
    pub path: String,
    pub topics: Vec<String>,
}