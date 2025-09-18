use foxglove_api::{ApiClient, ClientConfig, PostSitesRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sites_create_a_site(PostSitesRequest {
            name: "name",
            type_: "self-hosted",
        })
        .await;
}
