use foxglove_api::{ApiClient, ClientConfig, GetDataCoverageRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .coverage_list_coverage(GetDataCoverageRequest {})
        .await;
}
