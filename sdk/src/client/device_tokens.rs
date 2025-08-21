use crate::{ClientConfig, ClientError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct DeviceTokensClient {
    pub http_client: HttpClient,
}

impl DeviceTokensClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_device_tokens(&self, device_id: Option<String>, options: Option<RequestOptions>) -> Result<Vec<DeviceToken>, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            "device-tokens",
            None,
            {
            let mut query_params = Vec::new();
            if let Some(value) = device_id {
                query_params.push(("deviceId".to_string(), value.clone()));
            }
            Some(query_params)
        },
            options,
        ).await
    }

    pub async fn create_a_device_token(&self, request: &serde_json::Value, options: Option<RequestOptions>) -> Result<PostDeviceTokensResponse, ClientError> {
        self.http_client.execute_request(
            Method::POST,
            "device-tokens",
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

    pub async fn get_a_device_token(&self, id: &String, options: Option<RequestOptions>) -> Result<DeviceToken, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            &format!("device-tokens/{}", id),
            None,
            None,
            options,
        ).await
    }

    pub async fn delete_a_device_token(&self, id: &String, options: Option<RequestOptions>) -> Result<DeleteDeviceTokensIdResponse, ClientError> {
        self.http_client.execute_request(
            Method::DELETE,
            &format!("device-tokens/{}", id),
            None,
            None,
            options,
        ).await
    }

    pub async fn edit_a_device_token(&self, id: &String, request: &serde_json::Value, options: Option<RequestOptions>) -> Result<DeviceToken, ClientError> {
        self.http_client.execute_request(
            Method::PATCH,
            &format!("device-tokens/{}", id),
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

}

