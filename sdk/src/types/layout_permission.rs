use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LayoutPermission {
    #[serde(rename = "CREATOR_WRITE")]
    CreatorWrite,
    #[serde(rename = "ORG_READ")]
    OrgRead,
    #[serde(rename = "ORG_WRITE")]
    OrgWrite,
}
impl fmt::Display for LayoutPermission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CreatorWrite => "CREATOR_WRITE",
            Self::OrgRead => "ORG_READ",
            Self::OrgWrite => "ORG_WRITE",
        };
        write!(f, "{}", s)
    }
}
