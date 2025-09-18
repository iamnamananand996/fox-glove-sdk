use foxglove_api::{ApiClient, ClientConfig, GetRecordingAttachmentsRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<token>".to_string()),
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .recording_attachments_list_attachments(GetRecordingAttachmentsRequest {})
        .await;
}
