pub use crate::prelude::*;

/// The type of site to create.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostSitesRequestType {
    #[serde(rename = "self-hosted")]
    SelfHosted,
    #[serde(rename = "edge")]
    Edge,
}
impl fmt::Display for PostSitesRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SelfHosted => "self-hosted",
            Self::Edge => "edge",
        };
        write!(f, "{}", s)
    }
}
