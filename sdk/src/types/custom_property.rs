use crate::new_custom_property::NewCustomProperty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CustomProperty {
    #[serde(flatten)]
    pub new_custom_property_fields: NewCustomProperty,
    pub id: String,
}