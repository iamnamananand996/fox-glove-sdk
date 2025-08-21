use crate::{ClientConfig, ClientError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct LayoutsClient {
    pub http_client: HttpClient,
}

impl LayoutsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_layouts(&self, updated_since: Option<chrono::DateTime<chrono::Utc>>, include_data: Option<bool>, options: Option<RequestOptions>) -> Result<Vec<Layout>, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            "layouts",
            None,
            {
            let mut query_params = Vec::new();
            if let Some(value) = updated_since {
                query_params.push(("updatedSince".to_string(), value.to_rfc3339()));
            }
            if let Some(value) = include_data {
                query_params.push(("includeData".to_string(), serde_json::to_string(&value).unwrap_or_default()));
            }
            Some(query_params)
        },
            options,
        ).await
    }

}

