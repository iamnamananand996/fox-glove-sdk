use crate::api::types::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SiteTokensClient {
    pub http_client: HttpClient,
}

impl SiteTokensClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config)?,
        })
    }

    pub async fn list_site_tokens(
        &self,
        site_id: Option<String>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<SiteToken>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "site-tokens",
                None,
                QueryBuilder::new().string("siteId", site_id).build(),
                options,
            )
            .await
    }

    pub async fn create_a_site_token(
        &self,
        request: &serde_json::Value,
        options: Option<RequestOptions>,
    ) -> Result<PostSiteTokensResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "site-tokens",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn delete_a_site_token(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<GenericSuccess, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("site-tokens/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
