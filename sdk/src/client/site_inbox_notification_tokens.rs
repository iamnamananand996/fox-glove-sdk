use crate::{ClientConfig, ApiError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct SiteInboxNotificationTokensClient {
    pub http_client: HttpClient,
}

impl SiteInboxNotificationTokensClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_inbox_notification_tokens(&self, site_id: Option<String>, options: Option<RequestOptions>) -> Result<Vec<InboxNotificationToken>, ApiError> {
        self.http_client.execute_request(
            Method::GET,
            "site-inbox-notification-tokens",
            None,
            {
            let mut query_params = Vec::new();
            if let Some(value) = site_id {
                query_params.push(("siteId".to_string(), value.clone()));
            }
            Some(query_params)
        },
            options,
        ).await
    }

    pub async fn create_a_site_inbox_notification_token(&self, request: &serde_json::Value, options: Option<RequestOptions>) -> Result<PostSiteInboxNotificationTokensResponse, ApiError> {
        self.http_client.execute_request(
            Method::POST,
            "site-inbox-notification-tokens",
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

    pub async fn delete_an_inbox_notification_token(&self, id: &String, options: Option<RequestOptions>) -> Result<DeleteSiteInboxNotificationTokensIdResponse, ApiError> {
        self.http_client.execute_request(
            Method::DELETE,
            &format!("site-inbox-notification-tokens/{}", id),
            None,
            None,
            options,
        ).await
    }

}

