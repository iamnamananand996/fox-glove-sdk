use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CoverageClient {
    pub http_client: HttpClient,
}

impl CoverageClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_coverage(
        &self,
        request: &ListCoverageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Coverage>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "data/coverage",
                None,
                QueryBuilder::new()
                    .datetime("start", request.start.clone())
                    .datetime("end", request.end.clone())
                    .float("tolerance", request.tolerance.clone())
                    .string("deviceId", request.device_id.clone())
                    .string("deviceName", request.device_name.clone())
                    .bool(
                        "includeEdgeRecordings",
                        request.include_edge_recordings.clone(),
                    )
                    .string("importId", request.import_id.clone())
                    .string("recordingId", request.recording_id.clone())
                    .string("recordingKey", request.recording_key.clone())
                    .build(),
                options,
            )
            .await
    }
}
