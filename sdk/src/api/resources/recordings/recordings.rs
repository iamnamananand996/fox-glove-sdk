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
