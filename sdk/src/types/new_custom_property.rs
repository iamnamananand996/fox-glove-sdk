use crate::new_custom_property_value_type::NewCustomPropertyValueType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NewCustomProperty {
    pub key: String,
    pub label: String,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "valueType")]
    pub value_type: NewCustomPropertyValueType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}