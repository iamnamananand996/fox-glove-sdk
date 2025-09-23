use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RecordingAttachmentsClient {
    pub http_client: HttpClient,
}

impl RecordingAttachmentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_attachments(
        &self,
        request: &ListAttachmentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<RecordingAttachment>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "recording-attachments",
                None,
                QueryBuilder::new()
                    .string("recordingId", request.recording_id.clone())
                    .string("siteId", request.site_id.clone())
                    .string("deviceId", request.device_id.clone())
                    .string("deviceName", request.device_name.clone())
                    .serialize("sortBy", request.sort_by.clone())
                    .serialize("sortOrder", request.sort_order.clone())
                    .float("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
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
