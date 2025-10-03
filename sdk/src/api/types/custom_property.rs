pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CustomProperty {
    #[serde(flatten)]
    pub new_custom_property_fields: NewCustomProperty,
    /// ID of the custom property
    pub id: String,
}
