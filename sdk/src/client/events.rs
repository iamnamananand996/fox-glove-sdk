use crate::{ClientConfig, ApiError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct EventsClient {
    pub http_client: HttpClient,
}

impl EventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_events(&self, start: Option<chrono::DateTime<chrono::Utc>>, end: Option<chrono::DateTime<chrono::Utc>>, created_after: Option<chrono::DateTime<chrono::Utc>>, updated_after: Option<chrono::DateTime<chrono::Utc>>, device_id: Option<String>, device_name: Option<String>, query: Option<String>, sort_by: Option<GetEventsRequestSortBy>, sort_order: Option<GetEventsRequestSortOrder>, limit: Option<f64>, offset: Option<i32>, options: Option<RequestOptions>) -> Result<Vec<Event>, ApiError> {
        self.http_client.execute_request(
            Method::GET,
            "events",
            None,
            {
            let mut query_builder = crate::QueryParameterBuilder::new();
            if let Some(value) = start {
                query_builder.add_simple("start", &value.to_rfc3339());
            }
            if let Some(value) = end {
                query_builder.add_simple("end", &value.to_rfc3339());
            }
            if let Some(value) = created_after {
                query_builder.add_simple("createdAfter", &value.to_rfc3339());
            }
            if let Some(value) = updated_after {
                query_builder.add_simple("updatedAfter", &value.to_rfc3339());
            }
            if let Some(value) = device_id {
                query_builder.add_simple("deviceId", &value);
            }
            if let Some(value) = device_name {
                query_builder.add_simple("deviceName", &value);
            }
            if let Some(value) = query {
                // Try to parse as structured query, fall back to simple if it fails
                if let Err(_) = query_builder.add_structured_query(&value) {
                    query_builder.add_simple("query", &value);
                }
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

    pub async fn create_an_event(&self, request: &serde_json::Value, options: Option<RequestOptions>) -> Result<Event, ApiError> {
        self.http_client.execute_request(
            Method::POST,
            "events",
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

    pub async fn get_an_event(&self, id: &String, options: Option<RequestOptions>) -> Result<Event, ApiError> {
        self.http_client.execute_request(
            Method::GET,
            &format!("events/{}", id),
            None,
            None,
            options,
        ).await
    }

    pub async fn delete_an_event(&self, id: &String, options: Option<RequestOptions>) -> Result<DeleteEventsIdResponse, ApiError> {
        self.http_client.execute_request(
            Method::DELETE,
            &format!("events/{}", id),
            None,
            None,
            options,
        ).await
    }

    pub async fn update_an_event(&self, id: &String, request: &serde_json::Value, options: Option<RequestOptions>) -> Result<Event, ApiError> {
        self.http_client.execute_request(
            Method::PATCH,
            &format!("events/{}", id),
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

}

