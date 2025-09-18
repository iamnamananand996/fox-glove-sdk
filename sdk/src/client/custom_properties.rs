use crate::types::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CustomPropertiesClient {
    pub http_client: HttpClient,
}

impl CustomPropertiesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_custom_properties(
        &self,
        resource_type: Option<String>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<CustomProperty>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "custom-properties",
                None,
                QueryBuilder::new()
                    .serialize("resourceType", resource_type)
                    .build(),
                options,
            )
            .await
    }

    pub async fn create_a_custom_property(
        &self,
        request: &NewCustomProperty,
        options: Option<RequestOptions>,
    ) -> Result<CustomProperty, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "custom-properties",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn get_a_custom_property(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<CustomProperty, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("custom-properties/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn delete_a_custom_property(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<DeleteCustomPropertiesIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("custom-properties/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn edit_a_custom_property(
        &self,
        id: &String,
        request: &serde_json::Value,
        options: Option<RequestOptions>,
    ) -> Result<CustomProperty, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("custom-properties/{}", id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
