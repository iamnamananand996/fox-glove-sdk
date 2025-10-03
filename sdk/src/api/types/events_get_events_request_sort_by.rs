pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetEventsRequestSortBy {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "deviceId")]
    DeviceId,
    #[serde(rename = "deviceName")]
    DeviceName,
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "createdAt")]
    CreatedAt,
    #[serde(rename = "updatedAt")]
    UpdatedAt,
}
impl fmt::Display for GetEventsRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Id => "id",
            Self::DeviceId => "deviceId",
            Self::DeviceName => "deviceName",
            Self::Start => "start",
            Self::CreatedAt => "createdAt",
            Self::UpdatedAt => "updatedAt",
        };
        write!(f, "{}", s)
    }
}
