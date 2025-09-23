use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DevicesClient {
    pub http_client: HttpClient,
}

impl DevicesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_devices(
        &self,
        request: &ListDevicesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Device>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "devices",
                None,
                QueryBuilder::new()
                    .string("sortBy", request.sort_by.clone())
                    .structured_query("query", request.query.clone())
                    .serialize("sortOrder", request.sort_order.clone())
                    .float("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn create_a_device(
        &self,
        request: &PostDevicesRequest,
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
        request: &PatchDevicesNameOrIdRequest,
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
