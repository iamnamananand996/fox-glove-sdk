use crate::{ClientConfig, ClientError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct SitesClient {
    pub http_client: HttpClient,
}

impl SitesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_sites(&self, options: Option<RequestOptions>) -> Result<Vec<Site>, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            "sites",
            None,
            None,
            options,
        ).await
    }

    pub async fn create_a_site(&self, request: &serde_json::Value, options: Option<RequestOptions>) -> Result<Site, ClientError> {
        self.http_client.execute_request(
            Method::POST,
            "sites",
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

    pub async fn get_site_details(&self, id: &String, options: Option<RequestOptions>) -> Result<Site, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            &format!("sites/{}", id),
            None,
            None,
            options,
        ).await
    }

    pub async fn delete_a_site(&self, id: &String, options: Option<RequestOptions>) -> Result<DeleteSitesIdResponse, ClientError> {
        self.http_client.execute_request(
            Method::DELETE,
            &format!("sites/{}", id),
            None,
            None,
            options,
        ).await
    }

    pub async fn update_site_details(&self, id: &String, request: &serde_json::Value, options: Option<RequestOptions>) -> Result<Site, ClientError> {
        self.http_client.execute_request(
            Method::PATCH,
            &format!("sites/{}", id),
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

}

