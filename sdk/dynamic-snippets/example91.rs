use foxglove_api::{ApiClient, ClientConfig};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.device_tokens_delete_a_device_token("id").await;
}
