use foxglove_api::{ApiClient, ClientConfig, GetDevicesRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.devices_list_devices(GetDevicesRequest {}).await;
}
