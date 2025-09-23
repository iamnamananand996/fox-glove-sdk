use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EventsClient {
    pub http_client: HttpClient,
}

impl EventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_events(
        &self,
        request: &ListEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Event>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "events",
                None,
                QueryBuilder::new()
                    .datetime("start", request.start.clone())
                    .datetime("end", request.end.clone())
                    .datetime("createdAfter", request.created_after.clone())
                    .datetime("updatedAfter", request.updated_after.clone())
                    .string("deviceId", request.device_id.clone())
                    .string("deviceName", request.device_name.clone())
                    .structured_query("query", request.query.clone())
                    .serialize("sortBy", request.sort_by.clone())
                    .serialize("sortOrder", request.sort_order.clone())
                    .float("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn create_an_event(
        &self,
        request: &PostEventsRequest,
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
        request: &PatchEventsIdRequest,
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
