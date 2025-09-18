use foxglove_api::{ApiClient, ClientConfig, GetCustomPropertiesRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .custom_properties_list_custom_properties(GetCustomPropertiesRequest {})
        .await;
}
