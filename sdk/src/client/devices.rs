use crate::{ClientConfig, ClientError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct DevicesClient {
    pub http_client: HttpClient,
}

impl DevicesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_devices(&self, sort_by: Option<String>, query: Option<String>, sort_order: Option<GetDevicesRequestSortOrder>, limit: Option<f64>, offset: Option<i32>, options: Option<RequestOptions>) -> Result<Vec<Device>, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            "devices",
            None,
            {
            let mut query_builder = crate::QueryParameterBuilder::new();
            if let Some(value) = sort_by {
                query_builder.add_simple("sortBy", &value);
            }
            if let Some(value) = query {
                // Try to parse as structured query, fall back to simple if it fails
                if let Err(_) = query_builder.add_structured_query(&value) {
                    query_builder.add_simple("query", &value);
                }
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

    pub async fn create_a_device(&self, request: &serde_json::Value, options: Option<RequestOptions>) -> Result<Device, ClientError> {
        self.http_client.execute_request(
            Method::POST,
            "devices",
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

    pub async fn get_a_device(&self, name_or_id: &String, options: Option<RequestOptions>) -> Result<Device, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            &format!("devices/{}", name_or_id),
            None,
            None,
            options,
        ).await
    }

    pub async fn delete_a_device(&self, name_or_id: &String, options: Option<RequestOptions>) -> Result<DeleteDevicesNameOrIdResponse, ClientError> {
        self.http_client.execute_request(
            Method::DELETE,
            &format!("devices/{}", name_or_id),
            None,
            None,
            options,
        ).await
    }

    pub async fn update_a_device(&self, name_or_id: &String, request: &serde_json::Value, options: Option<RequestOptions>) -> Result<Device, ClientError> {
        self.http_client.execute_request(
            Method::PATCH,
            &format!("devices/{}", name_or_id),
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

}

