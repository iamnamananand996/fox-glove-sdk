use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LakeFilesClient {
    pub http_client: HttpClient,
}

impl LakeFilesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_lake_files(
        &self,
        request: &ListLakeFilesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GetLakeFilesResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "lake-files",
                None,
                QueryBuilder::new()
                    .string("siteId", request.site_id.clone())
                    .string("deviceId", request.device_id.clone())
                    .string("deviceName", request.device_name.clone())
                    .string("recordingId", request.recording_id.clone())
                    .string("recordingKey", request.recording_key.clone())
                    .datetime("start", request.start.clone())
                    .datetime("end", request.end.clone())
                    .string("topic", request.topic.clone())
                    .build(),
                options,
            )
            .await
    }
}
