use foxglove_api::{ApiClient, ClientConfig, PatchDevicesNameOrIdRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .devices_update_a_device("nameOrId", PatchDevicesNameOrIdRequest {})
        .await;
}
