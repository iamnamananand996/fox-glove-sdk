use foxglove_api::{ApiClient, ClientConfig, PostSiteTokensRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .site_tokens_create_a_site_token(PostSiteTokensRequest { site_id: "siteId" })
        .await;
}
