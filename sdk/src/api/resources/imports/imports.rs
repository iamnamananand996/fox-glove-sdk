use crate::api::types::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ImportsClient {
    pub http_client: HttpClient,
}

impl ImportsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config)?,
        })
    }

    pub async fn list_imports(
        &self,
        device_id: Option<String>,
        filename: Option<String>,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
        data_start: Option<chrono::DateTime<chrono::Utc>>,
        data_end: Option<chrono::DateTime<chrono::Utc>>,
        sort_by: Option<GetDataImportsRequestSortBy>,
        sort_order: Option<GetDataImportsRequestSortOrder>,
        limit: Option<f64>,
        offset: Option<i32>,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Import>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "data/imports",
                None,
                QueryBuilder::new()
                    .string("deviceId", device_id)
                    .string("filename", filename)
                    .datetime("start", start)
                    .datetime("end", end)
                    .datetime("dataStart", data_start)
                    .datetime("dataEnd", data_end)
                    .serialize("sortBy", sort_by)
                    .serialize("sortOrder", sort_order)
                    .float("limit", limit)
                    .int("offset", offset)
                    .build(),
                options,
            )
            .await
    }

    pub async fn delete_multiple_imports(
        &self,
        id: Option<String>,
        options: Option<RequestOptions>,
    ) -> Result<DeleteDataImportsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                "data/imports",
                None,
                QueryBuilder::new().string("id", id).build(),
                options,
            )
            .await
    }

    pub async fn delete_an_import(
        &self,
        import_id: &String,
        device_id: Option<String>,
        options: Option<RequestOptions>,
    ) -> Result<DeleteDataImportsImportIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("data/imports/{}", import_id),
                None,
                QueryBuilder::new().string("deviceId", device_id).build(),
                options,
            )
            .await
    }
}
