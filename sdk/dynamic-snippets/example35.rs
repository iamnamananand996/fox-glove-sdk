use foxglove_api::{ApiClient, ClientConfig, PostDataStreamRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .stream_data_download_data(PostDataStreamRequest {})
        .await;
}
