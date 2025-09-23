use crate::events_get_events_request_sort_by::GetEventsRequestSortBy;
use crate::events_get_events_request_sort_order::GetEventsRequestSortOrder;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "createdAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_after: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<GetEventsRequestSortBy>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<GetEventsRequestSortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}
