use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
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

    /// This endpoint returns a `link` URL where you can download your data as an `.mcap` or `.bag`
    /// file.
    ///
    /// To download your data:
    /// 1. Make a request to this endpoint.
    /// 2. Make a `GET` request to the `link` URL.
    ///
    /// One of `recordingId`, `key`, `importId` (deprecated) or all three of
    /// `deviceId`/`deviceName`, `start`, and `end` must be specified.
    ///
    /// _Note: You can only export a `.bag` file if you originally uploaded a `.bag` file._
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
