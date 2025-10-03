pub use crate::prelude::*;

/// The type of the site.
/// - `foxglove-hosted`: A [Foxglove-hosted Primary Site](https://docs.foxglove.dev/docs/primary-sites/introduction/#foxglove-hosted).
/// - `self-hosted`: A [self-hosted Primary Site](https://docs.foxglove.dev/docs/primary-sites/introduction/#self-hosted).
/// - `edge`: An [Edge Site](https://docs.foxglove.dev/docs/edge-sites/introduction/).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SiteType {
    #[serde(rename = "foxglove-hosted")]
    FoxgloveHosted,
    #[serde(rename = "self-hosted")]
    SelfHosted,
    #[serde(rename = "edge")]
    Edge,
}
impl fmt::Display for SiteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::FoxgloveHosted => "foxglove-hosted",
            Self::SelfHosted => "self-hosted",
            Self::Edge => "edge",
        };
        write!(f, "{}", s)
    }
}
