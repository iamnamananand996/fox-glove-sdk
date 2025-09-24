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

    /// A coverage range represents a time span for which Foxglove has data for a
    /// given device.
    ///
    /// Your must specify the `start` and `end` arguments when making a coverage request.
    ///
    /// Note: By default, only coverage ranges with imported recordings are returned. To include
    /// coverage ranges with unimported recordings from an Edge Site or Agent, set the
    /// `includeEdgeRecordings` query parameter to true
    ///
    /// # Arguments
    ///
    /// * `start` - Start of an inclusive time range
    /// * `end` - End of an inclusive time range
    /// * `tolerance` - Minimum interval (in seconds) that ranges must be separated by to be considered discrete.
    /// Currently, the minimum meaningful value is 14s and smaller values will be clamped to this value.
    /// * `device_id` - Filter coverage by device ID
    /// * `device_name` - Name of device associated with the data
    /// * `include_edge_recordings` - Include recordings from an Edge Site or Agent in the response.
    ///
    /// When edge recordings are included, each item in the response array will also include the
    /// `importStatus` for the coverage range.
    /// * `import_id` - Filter coverage by import ID
    /// * `recording_id` - Filter coverage by recording ID
    /// * `recording_key` - Filter coverage by recordingKey
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
