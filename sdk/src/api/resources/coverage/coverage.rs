use crate::api::types::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CoverageClient {
    pub http_client: HttpClient,
}

impl CoverageClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config)?,
        })
    }

    pub async fn list_coverage(
        &self,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
        tolerance: Option<f64>,
        device_id: Option<String>,
        device_name: Option<String>,
        include_edge_recordings: Option<bool>,
        import_id: Option<String>,
        recording_id: Option<String>,
        recording_key: Option<String>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Coverage>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "data/coverage",
                None,
                QueryBuilder::new()
                    .datetime("start", start)
                    .datetime("end", end)
                    .float("tolerance", tolerance)
                    .string("deviceId", device_id)
                    .string("deviceName", device_name)
                    .bool("includeEdgeRecordings", include_edge_recordings)
                    .string("importId", import_id)
                    .string("recordingId", recording_id)
                    .string("recordingKey", recording_key)
                    .build(),
                options,
            )
            .await
    }
}
