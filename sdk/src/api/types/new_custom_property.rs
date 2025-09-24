use crate::new_custom_property_value_type::NewCustomPropertyValueType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NewCustomProperty {
    /// An immutable identifier for this custom property; it cannot be edited once saved.
    ///
    /// This key is used to identify the custom property in queries; for example, when filtering
    /// your devices by a custom property ("partNumber:456").
    pub key: String,
    /// Display label for user interfaces. May be changed after saving.
    pub label: String,
    /// Resource type to which this custom property may be assigned
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// The type of the values associated with this custom property
    /// Number values will be treated as IEEE 754 64-bit floats.
    #[serde(rename = "valueType")]
    pub value_type: NewCustomPropertyValueType,
    /// Allowed string values; required if `valueType` is "enum"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}
