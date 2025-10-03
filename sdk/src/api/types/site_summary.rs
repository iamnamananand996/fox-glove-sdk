pub use crate::prelude::*;

/// ID and name of a site.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SiteSummary {
    pub id: String,
    pub name: String,
}
