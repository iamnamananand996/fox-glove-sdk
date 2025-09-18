use foxglove_api::{ApiClient, ClientConfig, GetSiteInboxNotificationTokensRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .site_inbox_notification_tokens_list_inbox_notification_tokens(
            GetSiteInboxNotificationTokensRequest {},
        )
        .await;
}
