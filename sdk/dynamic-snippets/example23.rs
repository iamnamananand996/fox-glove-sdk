use foxglove_api::{ApiClient, ClientConfig, GetDataImportsRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.imports_list_imports(GetDataImportsRequest {}).await;
}
