use foxglove_api::{ApiClient, ClientConfig, GetDeviceTokensRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .device_tokens_list_device_tokens(GetDeviceTokensRequest {})
        .await;
}
