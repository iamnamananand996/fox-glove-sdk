use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LayoutsClient {
    pub http_client: HttpClient,
}

impl LayoutsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the org layouts.
    ///
    /// Note: Only layouts shared with the org are returned in the response; no personal layouts are
    /// returned.
    ///
    /// # Arguments
    ///
    /// * `updated_since` - Return only layouts updated since this time.
    /// * `include_data` - When set to false, the `data` field is omitted from the response items.
    /// This can be used to limit bandwidth when querying many Layouts.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_layouts(
        &self,
        request: &ListLayoutsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Layout>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "layouts",
                None,
                QueryBuilder::new()
                    .datetime("updatedSince", request.updated_since.clone())
                    .bool("includeData", request.include_data.clone())
                    .build(),
                options,
            )
            .await
    }
}
