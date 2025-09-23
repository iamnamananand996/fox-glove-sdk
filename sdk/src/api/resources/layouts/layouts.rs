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
