use crate::{ClientConfig, ClientError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct RecordingsClient {
    pub http_client: HttpClient,
}

impl RecordingsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_recordings(&self, start: Option<chrono::DateTime<chrono::Utc>>, end: Option<chrono::DateTime<chrono::Utc>>, path: Option<String>, site_id: Option<String>, edge_site_id: Option<String>, device_id: Option<String>, device_name: Option<String>, topic: Option<String>, import_status: Option<GetRecordingsRequestImportStatus>, limit: Option<f64>, offset: Option<i32>, sort_by: Option<GetRecordingsRequestSortBy>, sort_order: Option<GetRecordingsRequestSortOrder>, options: Option<RequestOptions>) -> Result<Vec<Recording>, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            "recordings",
            None,
            {
            let mut query_builder = crate::QueryParameterBuilder::new();
            if let Some(value) = start {
                query_builder.add_simple("start", &value.to_rfc3339());
            }
            if let Some(value) = end {
                query_builder.add_simple("end", &value.to_rfc3339());
            }
            if let Some(value) = path {
                query_builder.add_simple("path", &value);
            }
            if let Some(value) = site_id {
                query_builder.add_simple("site.id", &value);
            }
            if let Some(value) = edge_site_id {
                query_builder.add_simple("edgeSite.id", &value);
            }
            if let Some(value) = device_id {
                query_builder.add_simple("deviceId", &value);
            }
            if let Some(value) = device_name {
                query_builder.add_simple("deviceName", &value);
            }
            if let Some(value) = topic {
                query_builder.add_simple("topic", &value);
            }
            if let Some(value) = import_status {
                query_builder.add_simple("importStatus", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = limit {
                query_builder.add_simple("limit", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = offset {
                query_builder.add_simple("offset", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = sort_by {
                query_builder.add_simple("sortBy", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = sort_order {
                query_builder.add_simple("sortOrder", &serde_json::to_string(&value).unwrap_or_default());
            }
            let params = query_builder.build();
            if params.is_empty() { None } else { Some(params) }
        },
            options,
        ).await
    }

    pub async fn get_a_recording(&self, key_or_id: &String, options: Option<RequestOptions>) -> Result<Recording, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            &format!("recordings/{}", key_or_id),
            None,
            None,
            options,
        ).await
    }

    pub async fn delete_a_recording(&self, key_or_id: &String, options: Option<RequestOptions>) -> Result<DeleteRecordingsKeyOrIdResponse, ClientError> {
        self.http_client.execute_request(
            Method::DELETE,
            &format!("recordings/{}", key_or_id),
            None,
            None,
            options,
        ).await
    }

    pub async fn import_from_edge(&self, key_or_id: &String, options: Option<RequestOptions>) -> Result<PostRecordingsKeyOrIdImportResponse, ClientError> {
        self.http_client.execute_request(
            Method::POST,
            &format!("recordings/{}", key_or_id),
            None,
            None,
            options,
        ).await
    }

    pub async fn list_pending_imports(&self, request_id: Option<String>, key: Option<String>, device_id: Option<String>, device_name: Option<String>, error: Option<String>, filename: Option<String>, updated_since: Option<chrono::DateTime<chrono::Utc>>, show_completed: Option<bool>, show_quarantined: Option<bool>, site_id: Option<String>, sort_by: Option<GetDataPendingImportsRequestSortBy>, sort_order: Option<GetDataPendingImportsRequestSortOrder>, limit: Option<f64>, offset: Option<i32>, options: Option<RequestOptions>) -> Result<Vec<PendingImport>, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            "data/pending-imports",
            None,
            {
            let mut query_builder = crate::QueryParameterBuilder::new();
            if let Some(value) = request_id {
                query_builder.add_simple("requestId", &value);
            }
            if let Some(value) = key {
                query_builder.add_simple("key", &value);
            }
            if let Some(value) = device_id {
                query_builder.add_simple("deviceId", &value);
            }
            if let Some(value) = device_name {
                query_builder.add_simple("deviceName", &value);
            }
            if let Some(value) = error {
                query_builder.add_simple("error", &value);
            }
            if let Some(value) = filename {
                query_builder.add_simple("filename", &value);
            }
            if let Some(value) = updated_since {
                query_builder.add_simple("updatedSince", &value.to_rfc3339());
            }
            if let Some(value) = show_completed {
                query_builder.add_simple("showCompleted", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = show_quarantined {
                query_builder.add_simple("showQuarantined", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = site_id {
                query_builder.add_simple("siteId", &value);
            }
            if let Some(value) = sort_by {
                query_builder.add_simple("sortBy", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = sort_order {
                query_builder.add_simple("sortOrder", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = limit {
                query_builder.add_simple("limit", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = offset {
                query_builder.add_simple("offset", &serde_json::to_string(&value).unwrap_or_default());
            }
            let params = query_builder.build();
            if params.is_empty() { None } else { Some(params) }
        },
            options,
        ).await
    }

    pub async fn upload_a_recording(&self, request: &serde_json::Value, options: Option<RequestOptions>) -> Result<PostDataUploadResponse, ClientError> {
        self.http_client.execute_request(
            Method::POST,
            "data/upload",
            Some(serde_json::to_value(request).unwrap_or_default()),
            None,
            options,
        ).await
    }

}

