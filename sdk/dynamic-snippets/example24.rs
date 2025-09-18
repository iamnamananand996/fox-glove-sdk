use foxglove_api::{ApiClient, ClientConfig, DeleteDataImportsRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .imports_delete_multiple_imports(DeleteDataImportsRequest {})
        .await;
}
