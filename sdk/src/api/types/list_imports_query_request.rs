use crate::imports_get_data_imports_request_sort_by::GetDataImportsRequestSortBy;
use crate::imports_get_data_imports_request_sort_order::GetDataImportsRequestSortOrder;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListImportsQueryRequest {
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "dataStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_start: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "dataEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_end: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<GetDataImportsRequestSortBy>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<GetDataImportsRequestSortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}
