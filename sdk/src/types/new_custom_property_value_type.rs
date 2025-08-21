use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NewCustomPropertyValueType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "enum")]
    Enum,
    #[serde(rename = "boolean")]
    Boolean,
}
impl fmt::Display for NewCustomPropertyValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::String => "string",
            Self::Number => "number",
            Self::Enum => "enum",
            Self::Boolean => "boolean",
        };
        write!(f, "{}", s)
    }
}
