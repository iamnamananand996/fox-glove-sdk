use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RecordingsClient {
    pub http_client: HttpClient,
}

impl RecordingsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_recordings(
        &self,
        request: &ListRecordingsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Recording>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "recordings",
                None,
                QueryBuilder::new()
                    .datetime("start", request.start.clone())
                    .datetime("end", request.end.clone())
                    .string("path", request.path.clone())
                    .string("site.id", request.site_id.clone())
                    .string("edgeSite.id", request.edge_site_id.clone())
                    .string("deviceId", request.device_id.clone())
                    .string("deviceName", request.device_name.clone())
                    .string("topic", request.topic.clone())
                    .serialize("importStatus", request.import_status.clone())
                    .float("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sortBy", request.sort_by.clone())
                    .serialize("sortOrder", request.sort_order.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get details on a specific recording.
    ///
    /// # Arguments
    ///
    /// * `key_or_id` - Recording Key or ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_a_recording(
        &self,
        key_or_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<Recording, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("recordings/{}", key_or_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a recording. Deleting a recording also deletes the data for that recording
    /// (including attachments, messages, metadata, etc).
    ///
    /// Note: For recordings stored at an Edge Site, this method deletes only
    /// the imported data for that recording, leaving the edge copy intact.
    ///
    /// # Arguments
    ///
    /// * `key_or_id` - Recording Key or ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_a_recording(
        &self,
        key_or_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<DeleteRecordingsKeyOrIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("recordings/{}", key_or_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Request import of a recording from an Edge Site to a Primary Site. Importing a recording
    /// makes the data (messages, metadata, attachments, etc.) available for download and streaming.
    ///
    /// If the recording is successfully queued for import, is already imported, or already queued for
    /// import, this endpoint will return a 200 response and include the recording ID and the
    /// `importStatus`.
    ///
    /// An import status of `complete` indicates the recording is already imported. Poll the `GET
    /// v1/recordings/{id}` endpoint to observe changes to the `importStatus`.
    ///
    /// If the recording cannot be found or is unavailable for import because the edge copy or site
    /// is deleted, this endpoint will return a 404 response.
    ///
    /// # Arguments
    ///
    /// * `key_or_id` - Recording ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn import_from_edge(
        &self,
        key_or_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<PostRecordingsKeyOrIdImportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("recordings/{}", key_or_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List the pending imports. These are in-progress import jobs for newly uploaded recordings.
    ///
    /// # Arguments
    ///
    /// * `request_id` - A specific import request ID
    /// * `key` - The unique key optionally provided when importing
    /// * `device_id` - ID of device associated with the pending import
    /// * `device_name` - Name of device associated with the pending import
    /// * `error` - A string to filter based on error messages
    /// * `filename` - Filename to exactly match
    /// * `updated_since` - Filter pending imports updated since this time
    /// * `show_completed` - Include completed requests
    /// * `show_quarantined` - Include quarantined requests
    /// * `site_id` - Filter response to imports at site with this ID
    /// * `sort_by` - Sort by a single field of the import type
    /// * `sort_order` - Sort order for the `sortBy` field
    /// * `limit` - Maximum number of items to return
    /// * `offset` - Number of items to skip before returning the results
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_pending_imports(
        &self,
        request: &ListPendingImportsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<PendingImport>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "data/pending-imports",
                None,
                QueryBuilder::new()
                    .string("requestId", request.request_id.clone())
                    .string("key", request.key.clone())
                    .string("deviceId", request.device_id.clone())
                    .string("deviceName", request.device_name.clone())
                    .string("error", request.error.clone())
                    .string("filename", request.filename.clone())
                    .datetime("updatedSince", request.updated_since.clone())
                    .bool("showCompleted", request.show_completed.clone())
                    .bool("showQuarantined", request.show_quarantined.clone())
                    .string("siteId", request.site_id.clone())
                    .serialize("sortBy", request.sort_by.clone())
                    .serialize("sortOrder", request.sort_order.clone())
                    .float("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Use this endpoint to upload data to your `foxglove-hosted` site. The upload is a two-request
    /// process.
    ///
    /// 1. Make a request to this upload endpoint to create an upload `link`.
    /// 2. Issue a PUT HTTP request to the `link` response field URL.
    ///
    /// _Your PUT request header should have `Content-Type: application/octet-stream`, and your
    /// request body should contain your file content._
    ///
    /// Note: If you are using a self-hosted site, see [this
    /// guide](https://docs.foxglove.dev/docs/primary-sites/self-hosting/manage-data) for uploading data.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn upload_a_recording(
        &self,
        request: &PostDataUploadRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostDataUploadResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "data/upload",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
