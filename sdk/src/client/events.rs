use crate::types::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EventsClient {
    pub http_client: HttpClient,
}

impl EventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_events(
        &self,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
        created_after: Option<chrono::DateTime<chrono::Utc>>,
        updated_after: Option<chrono::DateTime<chrono::Utc>>,
        device_id: Option<String>,
        device_name: Option<String>,
        query: Option<String>,
        sort_by: Option<GetEventsRequestSortBy>,
        sort_order: Option<GetEventsRequestSortOrder>,
        limit: Option<f64>,
        offset: Option<i32>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Event>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "events",
                None,
                QueryBuilder::new()
                    .datetime("start", start)
                    .datetime("end", end)
                    .datetime("createdAfter", created_after)
                    .datetime("updatedAfter", updated_after)
                    .string("deviceId", device_id)
                    .string("deviceName", device_name)
                    .structured_query("query", query)
                    .serialize("sortBy", sort_by)
                    .serialize("sortOrder", sort_order)
                    .float("limit", limit)
                    .int("offset", offset)
                    .build(),
                options,
            )
            .await
    }

    pub async fn create_an_event(
        &self,
        request: &serde_json::Value,
        options: Option<RequestOptions>,
    ) -> Result<Event, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "events",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn get_an_event(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<Event, ApiError> {
        self.http_client
            .execute_request(Method::GET, &format!("events/{}", id), None, None, options)
            .await
    }

    pub async fn delete_an_event(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<DeleteEventsIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("events/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn update_an_event(
        &self,
        id: &String,
        request: &serde_json::Value,
        options: Option<RequestOptions>,
    ) -> Result<Event, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("events/{}", id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
