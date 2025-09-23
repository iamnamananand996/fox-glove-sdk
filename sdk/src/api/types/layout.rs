use crate::layout_permission::LayoutPermission;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub id: String,
    pub name: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "savedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "folderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_name: Option<String>,
    pub permission: LayoutPermission,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
}
