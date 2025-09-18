use foxglove_api::{ApiClient, ClientConfig, GetLakeFilesRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .lake_files_list_lake_files(GetLakeFilesRequest { site_id: "siteId" })
        .await;
}
