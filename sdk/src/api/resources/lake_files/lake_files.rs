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

    /// This endpoint returns a list of MCAP files in the lake bucket for a given Primary Site.
    ///
    /// For each recording that has been imported, multiple files are created — one per topic.
    ///
    /// This endpoint is only supported for self-managed sites.
    ///
    /// A query must be limited to a device or recording using one of the following parameters:
    /// - deviceId
    /// - deviceName
    /// - recordingId
    /// - recordingKey
    ///
    /// If querying by a device (ID or name), you must also provide `start` and `end` parameters to limit the range of files included.
    ///
    /// The range expressed by `start` and `end` must not exceed 24h.
    ///
    /// # Arguments
    ///
    /// * `site_id` - The ID of a self-managed Primary Site for which the files have been imported
    /// * `device_id` - ID of the device associated with the imported files
    /// * `device_name` - Name of the device associated with the imported files
    /// * `recording_id` - A recording ID for which the files have been imported
    /// * `recording_key` - A recording key for which the files have been imported
    /// * `start` - Inclusive start of an imported recording's time range. If start is provided, end must be too.
    /// * `end` - Inclusive end of an imported recording's time range. If end is provided, start must be too.
    /// * `topic` - Include only imported files matching this topic name
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
