use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct StreamDataClient {
    pub http_client: HttpClient,
}

impl StreamDataClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn download_data(
        &self,
        request: &PostDataStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostDataStreamResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "data/stream",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
