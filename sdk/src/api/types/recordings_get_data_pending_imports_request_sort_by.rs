pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetDataPendingImportsRequestSortBy {
    #[serde(rename = "createdAt")]
    CreatedAt,
    #[serde(rename = "deviceId")]
    DeviceId,
    #[serde(rename = "deviceName")]
    DeviceName,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "importId")]
    ImportId,
    #[serde(rename = "requestId")]
    RequestId,
    #[serde(rename = "updatedAt")]
    UpdatedAt,
}
impl fmt::Display for GetDataPendingImportsRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CreatedAt => "createdAt",
            Self::DeviceId => "deviceId",
            Self::DeviceName => "deviceName",
            Self::Error => "error",
            Self::ImportId => "importId",
            Self::RequestId => "requestId",
            Self::UpdatedAt => "updatedAt",
        };
        write!(f, "{}", s)
    }
}
