pub use crate::prelude::*;

/// ID and name of a device.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeviceSummary {
    pub id: String,
    pub name: String,
}
