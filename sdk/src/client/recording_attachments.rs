use crate::types::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RecordingAttachmentsClient {
    pub http_client: HttpClient,
}

impl RecordingAttachmentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_attachments(
        &self,
        recording_id: Option<String>,
        site_id: Option<String>,
        device_id: Option<String>,
        device_name: Option<String>,
        sort_by: Option<String>,
        sort_order: Option<GetRecordingAttachmentsRequestSortOrder>,
        limit: Option<f64>,
        offset: Option<i32>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<RecordingAttachment>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "recording-attachments",
                None,
                QueryBuilder::new()
                    .string("recordingId", recording_id)
                    .string("siteId", site_id)
                    .string("deviceId", device_id)
                    .string("deviceName", device_name)
                    .serialize("sortBy", sort_by)
                    .serialize("sortOrder", sort_order)
                    .float("limit", limit)
                    .int("offset", offset)
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_an_attachment(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<RecordingAttachment, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("recording-attachments/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn download_an_attachment(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("recording-attachments/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
