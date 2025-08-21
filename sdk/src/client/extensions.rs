use crate::{ClientConfig, ClientError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct ExtensionsClient {
    pub http_client: HttpClient,
}

impl ExtensionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn publish_an_extension(&self, request: &Vec<u8>, options: Option<RequestOptions>) -> Result<PostExtensionUploadResponse, ClientError> {
        self.http_client.execute_request(
            Method::POST,
            "extension-upload",
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

    pub async fn list_extensions(&self, options: Option<RequestOptions>) -> Result<Vec<Extension>, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            "extensions",
            None,
            None,
            options,
        ).await
    }

    pub async fn get_an_extension(&self, id: &String, options: Option<RequestOptions>) -> Result<ExtensionWithSignedLink, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            &format!("extensions/{}", id),
            None,
            None,
            options,
        ).await
    }

    pub async fn delete_an_extension(&self, id: &String, options: Option<RequestOptions>) -> Result<DeleteExtensionsIdResponse, ClientError> {
        self.http_client.execute_request(
            Method::DELETE,
            &format!("extensions/{}", id),
            None,
            None,
            options,
        ).await
    }

}

