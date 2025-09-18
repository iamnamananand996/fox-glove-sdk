use crate::types::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TopicsClient {
    pub http_client: HttpClient,
}

impl TopicsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_topics(
        &self,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
        import_id: Option<String>,
        recording_id: Option<String>,
        recording_key: Option<String>,
        device_id: Option<String>,
        device_name: Option<String>,
        include_schemas: Option<bool>,
        sort_by: Option<GetDataTopicsRequestSortBy>,
        sort_order: Option<GetDataTopicsRequestSortOrder>,
        limit: Option<f64>,
        offset: Option<i32>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Topic>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "data/topics",
                None,
                QueryBuilder::new()
                    .datetime("start", start)
                    .datetime("end", end)
                    .string("importId", import_id)
                    .string("recordingId", recording_id)
                    .string("recordingKey", recording_key)
                    .string("deviceId", device_id)
                    .string("deviceName", device_name)
                    .bool("includeSchemas", include_schemas)
                    .serialize("sortBy", sort_by)
                    .serialize("sortOrder", sort_order)
                    .float("limit", limit)
                    .int("offset", offset)
                    .build(),
                options,
            )
            .await
    }
}
