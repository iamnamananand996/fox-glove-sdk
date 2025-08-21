use crate::{ClientConfig, ClientError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct CustomPropertiesClient {
    pub http_client: HttpClient,
}

impl CustomPropertiesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_custom_properties(&self, resource_type: Option<String>, options: Option<RequestOptions>) -> Result<Vec<CustomProperty>, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            "custom-properties",
            None,
            {
            let mut query_params = Vec::new();
            if let Some(value) = resource_type {
                query_params.push(("resourceType".to_string(), serde_json::to_string(&value).unwrap_or_default()));
            }
            Some(query_params)
        },
            options,
        ).await
    }

    pub async fn create_a_custom_property(&self, request: &NewCustomProperty, options: Option<RequestOptions>) -> Result<CustomProperty, ClientError> {
        self.http_client.execute_request(
            Method::POST,
            "custom-properties",
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

    pub async fn get_a_custom_property(&self, id: &String, options: Option<RequestOptions>) -> Result<CustomProperty, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            &format!("custom-properties/{}", id),
            None,
            None,
            options,
        ).await
    }

    pub async fn delete_a_custom_property(&self, id: &String, options: Option<RequestOptions>) -> Result<DeleteCustomPropertiesIdResponse, ClientError> {
        self.http_client.execute_request(
            Method::DELETE,
            &format!("custom-properties/{}", id),
            None,
            None,
            options,
        ).await
    }

    pub async fn edit_a_custom_property(&self, id: &String, request: &serde_json::Value, options: Option<RequestOptions>) -> Result<CustomProperty, ClientError> {
        self.http_client.execute_request(
            Method::PATCH,
            &format!("custom-properties/{}", id),
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

}

