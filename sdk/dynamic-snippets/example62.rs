use foxglove_api::{ApiClient, ClientConfig};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .custom_properties_create_a_custom_property(
            serde_json::json!({"key":"x","label":"x","resourceType":"device","valueType":"string"}),
        )
        .await;
}
