use foxglove_api::{ApiClient, ClientConfig, GetEventsRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.events_list_events(GetEventsRequest {}).await;
}
