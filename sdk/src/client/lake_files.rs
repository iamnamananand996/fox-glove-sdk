use crate::{ClientConfig, ApiError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct LakeFilesClient {
    pub http_client: HttpClient,
}

impl LakeFilesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_lake_files(&self, site_id: Option<String>, device_id: Option<String>, device_name: Option<String>, recording_id: Option<String>, recording_key: Option<String>, start: Option<chrono::DateTime<chrono::Utc>>, end: Option<chrono::DateTime<chrono::Utc>>, topic: Option<String>, options: Option<RequestOptions>) -> Result<Vec<GetLakeFilesResponseItem>, ApiError> {
        self.http_client.execute_request(
            Method::GET,
            "lake-files",
            None,
            {
            let mut query_builder = crate::QueryParameterBuilder::new();
            if let Some(value) = site_id {
                query_builder.add_simple("siteId", &value);
            }
            if let Some(value) = device_id {
                query_builder.add_simple("deviceId", &value);
            }
            if let Some(value) = device_name {
                query_builder.add_simple("deviceName", &value);
            }
            if let Some(value) = recording_id {
                query_builder.add_simple("recordingId", &value);
            }
            if let Some(value) = recording_key {
                query_builder.add_simple("recordingKey", &value);
            }
            if let Some(value) = start {
                query_builder.add_simple("start", &value.to_rfc3339());
            }
            if let Some(value) = end {
                query_builder.add_simple("end", &value.to_rfc3339());
            }
            if let Some(value) = topic {
                query_builder.add_simple("topic", &value);
            }
            let params = query_builder.build();
            if params.is_empty() { None } else { Some(params) }
        },
            options,
        ).await
    }

}

