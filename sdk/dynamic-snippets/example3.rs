use foxglove_api::{ApiClient, ClientConfig, PostDevicesRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .devices_create_a_device(PostDevicesRequest { name: "name" })
        .await;
}
