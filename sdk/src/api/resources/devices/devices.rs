use crate::api::types::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DevicesClient {
    pub http_client: HttpClient,
}

impl DevicesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config)?,
        })
    }

    pub async fn list_devices(
        &self,
        sort_by: Option<String>,
        query: Option<String>,
        sort_order: Option<GetDevicesRequestSortOrder>,
        limit: Option<f64>,
        offset: Option<i32>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Device>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "devices",
                None,
                QueryBuilder::new()
                    .string("sortBy", sort_by)
                    .structured_query("query", query)
                    .serialize("sortOrder", sort_order)
                    .float("limit", limit)
                    .int("offset", offset)
                    .build(),
                options,
            )
            .await
    }

    pub async fn create_a_device(
        &self,
        request: &serde_json::Value,
        options: Option<RequestOptions>,
    ) -> Result<Device, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "devices",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn get_a_device(
        &self,
        name_or_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<Device, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("devices/{}", name_or_id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn delete_a_device(
        &self,
        name_or_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<DeleteDevicesNameOrIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("devices/{}", name_or_id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn update_a_device(
        &self,
        name_or_id: &String,
        request: &serde_json::Value,
        options: Option<RequestOptions>,
    ) -> Result<Device, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("devices/{}", name_or_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
