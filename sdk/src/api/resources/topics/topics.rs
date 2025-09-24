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

    /// Get a list of topics available for a device within a given time range.
    ///
    /// By default, this endpoint will not return the `schema` for each topic. To include
    /// the schemas, you must provide the `includeSchemas` query parameter.
    ///
    /// Use `start` and `end` to limit the response to overlapping recording ranges.
    ///
    /// Topics for not-imported recordings are only returned if no parameter is provided besides
    /// recordingId or recordingKey. This is because most parameters need the imported files to
    /// filter, and can only return an empty list if imports are unavailable.
    ///
    /// # Arguments
    ///
    /// * `start` - Start of an inclusive time range
    /// * `end` - End of an inclusive time range
    /// * `import_id` - ID of the import from which to list topics
    /// * `recording_id` - ID of the recording from which to list topics
    /// * `recording_key` - Key of the recording from which to list topics
    /// * `device_id` - ID of device being queried
    /// * `device_name` - Name of device being queried
    /// * `include_schemas` - Whether full schemas should be included in the response
    /// * `sort_by` - Sort by a single field of the topic type ("topic" or "version")
    /// * `sort_order` - Sort order for the `sortBy` field
    /// * `limit` - Maximum number of items to return
    /// * `offset` - Number of items to skip before returning the results
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
