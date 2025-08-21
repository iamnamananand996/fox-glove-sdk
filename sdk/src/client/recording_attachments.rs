use crate::{ClientConfig, ClientError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct RecordingAttachmentsClient {
    pub http_client: HttpClient,
}

impl RecordingAttachmentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_attachments(&self, recording_id: Option<String>, site_id: Option<String>, device_id: Option<String>, device_name: Option<String>, sort_by: Option<String>, sort_order: Option<GetRecordingAttachmentsRequestSortOrder>, limit: Option<f64>, offset: Option<i32>, options: Option<RequestOptions>) -> Result<Vec<RecordingAttachment>, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            "recording-attachments",
            None,
            {
            let mut query_builder = crate::QueryParameterBuilder::new();
            if let Some(value) = recording_id {
                query_builder.add_simple("recordingId", &value);
            }
            if let Some(value) = site_id {
                query_builder.add_simple("siteId", &value);
            }
            if let Some(value) = device_id {
                query_builder.add_simple("deviceId", &value);
            }
            if let Some(value) = device_name {
                query_builder.add_simple("deviceName", &value);
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

    pub async fn get_an_attachment(&self, id: &String, options: Option<RequestOptions>) -> Result<RecordingAttachment, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            &format!("recording-attachments/{}", id),
            None,
            None,
            options,
        ).await
    }

    pub async fn download_an_attachment(&self, id: &String, options: Option<RequestOptions>) -> Result<(), ClientError> {
        self.http_client.execute_request(
            Method::GET,
            &format!("recording-attachments/{}", id),
            None,
            None,
            options,
        ).await
    }

}

