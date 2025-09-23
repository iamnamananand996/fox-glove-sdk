use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ImportsClient {
    pub http_client: HttpClient,
}

impl ImportsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_imports(
        &self,
        request: &ListImportsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Import>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "data/imports",
                None,
                QueryBuilder::new()
                    .string("deviceId", request.device_id.clone())
                    .string("filename", request.filename.clone())
                    .datetime("start", request.start.clone())
                    .datetime("end", request.end.clone())
                    .datetime("dataStart", request.data_start.clone())
                    .datetime("dataEnd", request.data_end.clone())
                    .serialize("sortBy", request.sort_by.clone())
                    .serialize("sortOrder", request.sort_order.clone())
                    .float("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn delete_multiple_imports(
        &self,
        request: &DeleteMultipleImportsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteDataImportsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                "data/imports",
                None,
                QueryBuilder::new().string("id", request.id.clone()).build(),
                options,
            )
            .await
    }

    pub async fn delete_an_import(
        &self,
        import_id: &String,
        request: &DeleteAnImportQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteDataImportsImportIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("data/imports/{}", import_id),
                None,
                QueryBuilder::new()
                    .string("deviceId", request.device_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
