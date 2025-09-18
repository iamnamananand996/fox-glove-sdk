use foxglove_api::{ApiClient, ClientConfig, PatchDeviceTokensIdRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .device_tokens_edit_a_device_token("id", PatchDeviceTokensIdRequest { enabled: true })
        .await;
}
