use foxglove_api::{ApiClient, ClientConfig, PostDeviceTokensRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .device_tokens_create_a_device_token(PostDeviceTokensRequest {
            device_id: "deviceId",
        })
        .await;
}
