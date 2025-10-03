pub use crate::prelude::*;

/// If set to "lastPerChannel", then the stream will include the most recent message
/// on each channel, even if it comes before the requested `start`, as long as it is
/// within the window of `replayLookbackSeconds` seconds before `start`.
///
/// The default, `""` (no policy), means no messages before `start` are included.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostDataStreamRequestReplayPolicy {
    #[serde(rename = "lastPerChannel")]
    LastPerChannel,
    #[serde(rename = "")]
    Empty,
}
impl fmt::Display for PostDataStreamRequestReplayPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::LastPerChannel => "lastPerChannel",
            Self::Empty => "",
        };
        write!(f, "{}", s)
    }
}
