use foxglove_api::{ApiClient, ClientConfig, PostEventsRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .events_create_an_event(PostEventsRequest {
            start: todo!("Unhandled primitive: DATE_TIME"),
            end: todo!("Unhandled primitive: DATE_TIME"),
        })
        .await;
}
