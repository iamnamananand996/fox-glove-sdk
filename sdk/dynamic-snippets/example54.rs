use foxglove_api::{ApiClient, ClientConfig, PatchEventsIdRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .events_update_an_event("id", PatchEventsIdRequest {})
        .await;
}
