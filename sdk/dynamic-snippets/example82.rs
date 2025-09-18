use foxglove_api::{ApiClient, ClientConfig, PostSiteInboxNotificationTokensRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .site_inbox_notification_tokens_create_a_site_inbox_notification_token(
            PostSiteInboxNotificationTokensRequest { site_id: "siteId" },
        )
        .await;
}
