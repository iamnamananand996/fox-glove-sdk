use crate::{ClientConfig, ClientError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct CoverageClient {
    pub http_client: HttpClient,
}

impl CoverageClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_coverage(&self, start: Option<chrono::DateTime<chrono::Utc>>, end: Option<chrono::DateTime<chrono::Utc>>, tolerance: Option<f64>, device_id: Option<String>, device_name: Option<String>, include_edge_recordings: Option<bool>, import_id: Option<String>, recording_id: Option<String>, recording_key: Option<String>, options: Option<RequestOptions>) -> Result<Vec<Coverage>, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            "data/coverage",
            None,
            {
            let mut query_builder = crate::QueryParameterBuilder::new();
            if let Some(value) = start {
                query_builder.add_simple("start", &value.to_rfc3339());
            }
            if let Some(value) = end {
                query_builder.add_simple("end", &value.to_rfc3339());
            }
            if let Some(value) = tolerance {
                query_builder.add_simple("tolerance", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = device_id {
                query_builder.add_simple("deviceId", &value);
            }
            if let Some(value) = device_name {
                query_builder.add_simple("deviceName", &value);
            }
            if let Some(value) = include_edge_recordings {
                query_builder.add_simple("includeEdgeRecordings", &serde_json::to_string(&value).unwrap_or_default());
            }
            if let Some(value) = import_id {
                query_builder.add_simple("importId", &value);
            }
            if let Some(value) = recording_id {
                query_builder.add_simple("recordingId", &value);
            }
            if let Some(value) = recording_key {
                query_builder.add_simple("recordingKey", &value);
            }
            let params = query_builder.build();
            if params.is_empty() { None } else { Some(params) }
        },
            options,
        ).await
    }

}

