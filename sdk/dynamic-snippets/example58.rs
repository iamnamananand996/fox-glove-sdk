use foxglove_api::{ApiClient, ClientConfig, GetLayoutsRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.layouts_list_layouts(GetLayoutsRequest {}).await;
}
