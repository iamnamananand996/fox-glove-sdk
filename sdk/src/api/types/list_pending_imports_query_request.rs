use crate::recordings_get_data_pending_imports_request_sort_by::GetDataPendingImportsRequestSortBy;
use crate::recordings_get_data_pending_imports_request_sort_order::GetDataPendingImportsRequestSortOrder;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPendingImportsQueryRequest {
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "updatedSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_since: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "showCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_completed: Option<bool>,
    #[serde(rename = "showQuarantined")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_quarantined: Option<bool>,
    #[serde(rename = "siteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<GetDataPendingImportsRequestSortBy>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<GetDataPendingImportsRequestSortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}
