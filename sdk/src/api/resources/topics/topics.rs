use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TopicsClient {
    pub http_client: HttpClient,
}

impl TopicsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_topics(
        &self,
        request: &ListTopicsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Topic>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "data/topics",
                None,
                QueryBuilder::new()
                    .datetime("start", request.start.clone())
                    .datetime("end", request.end.clone())
                    .string("importId", request.import_id.clone())
                    .string("recordingId", request.recording_id.clone())
                    .string("recordingKey", request.recording_key.clone())
                    .string("deviceId", request.device_id.clone())
                    .string("deviceName", request.device_name.clone())
                    .bool("includeSchemas", request.include_schemas.clone())
                    .serialize("sortBy", request.sort_by.clone())
                    .serialize("sortOrder", request.sort_order.clone())
                    .float("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }
}
