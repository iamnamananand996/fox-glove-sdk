use crate::types::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DeviceTokensClient {
    pub http_client: HttpClient,
}

impl DeviceTokensClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_device_tokens(
        &self,
        device_id: Option<String>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<DeviceToken>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "device-tokens",
                None,
                QueryBuilder::new().string("deviceId", device_id).build(),
                options,
            )
            .await
    }

    pub async fn create_a_device_token(
        &self,
        request: &serde_json::Value,
        options: Option<RequestOptions>,
    ) -> Result<PostDeviceTokensResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "device-tokens",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn get_a_device_token(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<DeviceToken, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("device-tokens/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn delete_a_device_token(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<DeleteDeviceTokensIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("device-tokens/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn edit_a_device_token(
        &self,
        id: &String,
        request: &serde_json::Value,
        options: Option<RequestOptions>,
    ) -> Result<DeviceToken, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("device-tokens/{}", id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
