use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExtensionWithSignedLink {
    /// Signed download link for the extension
    pub foxe: String,
}
