pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Site {
    /// Unique identifier for this site
    pub id: String,
    /// Name for this site
    pub name: String,
    /// The type of the site.
    /// - `foxglove-hosted`: A [Foxglove-hosted Primary Site](https://docs.foxglove.dev/docs/primary-sites/introduction/#foxglove-hosted).
    /// - `self-hosted`: A [self-hosted Primary Site](https://docs.foxglove.dev/docs/primary-sites/introduction/#self-hosted).
    /// - `edge`: An [Edge Site](https://docs.foxglove.dev/docs/edge-sites/introduction/).
    pub r#type: SiteType,
    /// The REST API endpoint your site deployment exposes
    /// (only available for self-hosted Primary Sites).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// An optional retention period for recordings created at this site. If set to zero, recordings are retained indefinitely. (only available on Edge Sites)
    #[serde(rename = "retainRecordingsSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_recordings_seconds: Option<f64>,
}
