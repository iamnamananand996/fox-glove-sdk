use crate::types::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LakeFilesClient {
    pub http_client: HttpClient,
}

impl LakeFilesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_lake_files(
        &self,
        site_id: Option<String>,
        device_id: Option<String>,
        device_name: Option<String>,
        recording_id: Option<String>,
        recording_key: Option<String>,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
        topic: Option<String>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GetLakeFilesResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "lake-files",
                None,
                QueryBuilder::new()
                    .string("siteId", site_id)
                    .string("deviceId", device_id)
                    .string("deviceName", device_name)
                    .string("recordingId", recording_id)
                    .string("recordingKey", recording_key)
                    .datetime("start", start)
                    .datetime("end", end)
                    .string("topic", topic)
                    .build(),
                options,
            )
            .await
    }
}
