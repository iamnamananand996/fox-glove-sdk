use crate::{ClientConfig, ClientError, HttpClient, RequestOptions};
use reqwest::{Method};
use crate::{types::*};

pub struct ImportsClient {
    pub http_client: HttpClient,
}

impl ImportsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub async fn list_imports(&self, device_id: Option<String>, filename: Option<String>, start: Option<chrono::DateTime<chrono::Utc>>, end: Option<chrono::DateTime<chrono::Utc>>, data_start: Option<chrono::DateTime<chrono::Utc>>, data_end: Option<chrono::DateTime<chrono::Utc>>, sort_by: Option<GetDataImportsRequestSortBy>, sort_order: Option<GetDataImportsRequestSortOrder>, limit: Option<f64>, offset: Option<i32>, options: Option<RequestOptions>) -> Result<Vec<Import>, ClientError> {
        self.http_client.execute_request(
            Method::GET,
            "data/imports",
            None,
            {
            let mut query_builder = crate::QueryParameterBuilder::new();
            if let Some(value) = device_id {
                query_builder.add_simple("deviceId", &value);
            }
            if let Some(value) = filename {
                query_builder.add_simple("filename", &value);
            }
            if let Some(value) = start {
                query_builder.add_simple("start", &value.to_rfc3339());
            }
            if let Some(value) = end {
                query_builder.add_simple("end", &value.to_rfc3339());
            }
            if let Some(value) = data_start {
                query_builder.add_simple("dataStart", &value.to_rfc3339());
            }
            if let Some(value) = data_end {
                query_builder.add_simple("dataEnd", &value.to_rfc3339());
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

    pub async fn delete_multiple_imports(&self, id: Option<String>, options: Option<RequestOptions>) -> Result<DeleteDataImportsResponse, ClientError> {
        self.http_client.execute_request(
            Method::DELETE,
            "data/imports",
            None,
            {
            let mut query_params = Vec::new();
            if let Some(value) = id {
                query_params.push(("id".to_string(), value.clone()));
            }
            Some(query_params)
        },
            options,
        ).await
    }

    pub async fn delete_an_import(&self, import_id: &String, device_id: Option<String>, options: Option<RequestOptions>) -> Result<DeleteDataImportsImportIdResponse, ClientError> {
        self.http_client.execute_request(
            Method::DELETE,
            &format!("data/imports/{}", import_id),
            None,
            {
            let mut query_params = Vec::new();
            if let Some(value) = device_id {
                query_params.push(("deviceId".to_string(), value.clone()));
            }
            Some(query_params)
        },
            options,
        ).await
    }

}

