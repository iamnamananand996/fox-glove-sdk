use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SiteInboxNotificationTokensClient {
    pub http_client: HttpClient,
}

impl SiteInboxNotificationTokensClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_inbox_notification_tokens(
        &self,
        request: &ListInboxNotificationTokensQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<InboxNotificationToken>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "site-inbox-notification-tokens",
                None,
                QueryBuilder::new()
                    .string("siteId", request.site_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn create_a_site_inbox_notification_token(
        &self,
        request: &PostSiteInboxNotificationTokensRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostSiteInboxNotificationTokensResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "site-inbox-notification-tokens",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn delete_an_inbox_notification_token(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<DeleteSiteInboxNotificationTokensIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("site-inbox-notification-tokens/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
