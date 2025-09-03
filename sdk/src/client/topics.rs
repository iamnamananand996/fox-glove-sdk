use crate::{ClientConfig, ApiError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct TopicsClient {
    pub http_client: HttpClient,
}

impl TopicsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_topics(&self, start: Option<chrono::DateTime<chrono::Utc>>, end: Option<chrono::DateTime<chrono::Utc>>, import_id: Option<String>, recording_id: Option<String>, recording_key: Option<String>, device_id: Option<String>, device_name: Option<String>, include_schemas: Option<bool>, sort_by: Option<GetDataTopicsRequestSortBy>, sort_order: Option<GetDataTopicsRequestSortOrder>, limit: Option<f64>, offset: Option<i32>, options: Option<RequestOptions>) -> Result<Vec<Topic>, ApiError> {
        self.http_client.execute_request(
            Method::GET,
            "data/topics",
            None,
            {
            let mut query_builder = crate::QueryParameterBuilder::new();
            if let Some(value) = start {
                query_builder.add_simple("start", &value.to_rfc3339());
            }
            if let Some(value) = end {
                query_builder.add_simple("end", &value.to_rfc3339());
            }
            if let Some(value) = import_id {
                query_builder.add_simple("importId", &value);
            }
            if let Some(value) = recording_id {
                query_builder.add_simple("recordingId", &value);
            }
            if let Some(value) = recording_key {
                query_builder.add_simple("recordingKey", &value);
            }
            if let Some(value) = device_id {
                query_builder.add_simple("deviceId", &value);
            }
            if let Some(value) = device_name {
                query_builder.add_simple("deviceName", &value);
            }
            if let Some(value) = include_schemas {
                query_builder.add_simple("includeSchemas", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = sort_by {
                query_builder.add_simple("sortBy", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = sort_order {
                query_builder.add_simple("sortOrder", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = limit {
                query_builder.add_simple("limit", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = offset {
                query_builder.add_simple("offset", &serde_json::to_string(&value).unwrap_or_default());
            }
            let params = query_builder.build();
            if params.is_empty() { None } else { Some(params) }
        },
            options,
        ).await
    }

}

