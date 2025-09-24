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

    /// **This endpoint is deprecated. Use the [list recordings](#tag/Recordings/paths/~1recordings/get) endpoint instead.**
    ///
    /// # Arguments
    ///
    /// * `device_id` - ID of device associated with the exported data
    /// * `filename` - Filename to match
    /// * `start` - Start of an inclusive time range
    /// * `end` - End of an inclusive time range
    /// * `data_start` - Inclusive start of message log time
    /// * `data_end` - Inclusive end of message log time
    /// * `sort_by` - Sort by a single field of the import type
    /// * `sort_order` - Sort order for the `sortBy` field
    /// * `limit` - Maximum number of items to return
    /// * `offset` - Number of items to skip before returning the results
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Deletes multiple imports by ID. Returns an array of result objects, which indicate whether a given import was successfully deleted. An import that has already been deleted will result in "notFound".
    /// _Note: All imports must belong to the same site. If any import belongs to a different site, the entire request is rejected with a 400 response._
    ///
    /// # Arguments
    ///
    /// * `id` - ID of import to delete. You can specify up to 50 IDs (for example, `?id=abc&id=def&...`).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// **This endpoint is deprecated. Use the [delete recording](#tag/Recordings/paths/~1recordings~1%7Bid%7D/delete) endpoint instead.**
    ///
    /// Deleting an import deletes all data associated with the import.
    ///
    /// **This action is permanent and cannot be undone.**
    ///
    /// # Arguments
    ///
    /// * `import_id` - The `importId` of an import
    /// * `device_id` - The deviceId from the import record
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
