use crate::api::types::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LayoutsClient {
    pub http_client: HttpClient,
}

impl LayoutsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config)?,
        })
    }

    pub async fn list_layouts(
        &self,
        updated_since: Option<chrono::DateTime<chrono::Utc>>,
        include_data: Option<bool>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Layout>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "layouts",
                None,
                QueryBuilder::new()
                    .datetime("updatedSince", updated_since)
                    .bool("includeData", include_data)
                    .build(),
                options,
            )
            .await
    }
}
