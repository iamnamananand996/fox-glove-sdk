use crate::device_name::DeviceName;
use crate::devices_post_devices_request_properties_value::PostDevicesRequestPropertiesValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostDevicesRequest {
    pub name: DeviceName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, PostDevicesRequestPropertiesValue>>,
}
