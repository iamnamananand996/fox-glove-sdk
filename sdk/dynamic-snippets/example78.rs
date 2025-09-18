use foxglove_api::{ApiClient, ClientConfig, PatchSitesIdRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sites_update_site_details("id", PatchSitesIdRequest {})
        .await;
}
