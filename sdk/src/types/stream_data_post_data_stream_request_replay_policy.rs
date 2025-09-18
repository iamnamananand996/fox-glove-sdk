use serde::{Deserialize, Serialize};
use std::fmt;

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
