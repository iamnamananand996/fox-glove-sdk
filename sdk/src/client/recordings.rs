use crate::types::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RecordingsClient {
    pub http_client: HttpClient,
}

impl RecordingsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_recordings(
        &self,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
        path: Option<String>,
        site_id: Option<String>,
        edge_site_id: Option<String>,
        device_id: Option<String>,
        device_name: Option<String>,
        topic: Option<String>,
        import_status: Option<GetRecordingsRequestImportStatus>,
        limit: Option<f64>,
        offset: Option<i32>,
        sort_by: Option<GetRecordingsRequestSortBy>,
        sort_order: Option<GetRecordingsRequestSortOrder>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Recording>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "recordings",
                None,
                QueryBuilder::new()
                    .datetime("start", start)
                    .datetime("end", end)
                    .string("path", path)
                    .string("site.id", site_id)
                    .string("edgeSite.id", edge_site_id)
                    .string("deviceId", device_id)
                    .string("deviceName", device_name)
                    .string("topic", topic)
                    .serialize("importStatus", import_status)
                    .float("limit", limit)
                    .int("offset", offset)
                    .serialize("sortBy", sort_by)
                    .serialize("sortOrder", sort_order)
                    .build(),
                options,
            )
            .await
    }

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

    pub async fn list_pending_imports(
        &self,
        request_id: Option<String>,
        key: Option<String>,
        device_id: Option<String>,
        device_name: Option<String>,
        error: Option<String>,
        filename: Option<String>,
        updated_since: Option<chrono::DateTime<chrono::Utc>>,
        show_completed: Option<bool>,
        show_quarantined: Option<bool>,
        site_id: Option<String>,
        sort_by: Option<GetDataPendingImportsRequestSortBy>,
        sort_order: Option<GetDataPendingImportsRequestSortOrder>,
        limit: Option<f64>,
        offset: Option<i32>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<PendingImport>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "data/pending-imports",
                None,
                QueryBuilder::new()
                    .string("requestId", request_id)
                    .string("key", key)
                    .string("deviceId", device_id)
                    .string("deviceName", device_name)
                    .string("error", error)
                    .string("filename", filename)
                    .datetime("updatedSince", updated_since)
                    .bool("showCompleted", show_completed)
                    .bool("showQuarantined", show_quarantined)
                    .string("siteId", site_id)
                    .serialize("sortBy", sort_by)
                    .serialize("sortOrder", sort_order)
                    .float("limit", limit)
                    .int("offset", offset)
                    .build(),
                options,
            )
            .await
    }

    pub async fn upload_a_recording(
        &self,
        request: &serde_json::Value,
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
