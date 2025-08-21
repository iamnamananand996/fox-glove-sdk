use crate::{ClientConfig, ClientError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct StreamDataClient {
    pub http_client: HttpClient,
}

impl StreamDataClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn download_data(&self, request: &serde_json::Value, options: Option<RequestOptions>) -> Result<PostDataStreamResponse, ClientError> {
        self.http_client.execute_request(
            Method::POST,
            "data/stream",
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

}

