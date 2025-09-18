use foxglove_api::{ApiClient, ClientConfig, GetRecordingsRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .recordings_list_recordings(GetRecordingsRequest {})
        .await;
}
