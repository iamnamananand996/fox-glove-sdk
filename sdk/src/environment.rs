use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "production")]
    Production,
}
impl Environment {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Production => "https://api.foxglove.dev/v1",
        }
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::Production
    }
}
