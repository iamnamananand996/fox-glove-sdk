use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DevicesClient {
    pub http_client: HttpClient,
}

impl DevicesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a list of devices.
    ///
    /// **Filtering by custom properties**
    ///
    /// Use the `query` parameter to filter devices on custom properties.
    /// Syntax:
    /// * `properties.key:value`: matches devices with a property that contains a key named `key` with a value of `value`; use double quotes if the value contains spaces or special characters
    /// * `properties.key:value1,value2`: matches devices with a property that contains a key named `key` and its value is either `value1` or `value2`
    /// * `properties.key:*`: matches devices with a property that contains a key named `key` and any value
    /// * `foo`: matches devices with properties where any key or stringified value contains `foo`
    /// Multiple qualifiers can be used in the same query string; this will filter devices matching the intersection of the qualifiers (AND).
    ///
    /// # Arguments
    ///
    /// * `sort_by` - Field to sort items by ("id", "name", or a custom property key prefixed with `properties.`)
    /// * `query` - Space-separated query string for device custom properties. Each custom property key must be valid and prefixed with "properties.". See above for syntax and examples.
    /// * `sort_order` - Sort order for the `sortBy` field
    /// * `limit` - Maximum number of items to return
    /// * `offset` - Number of items to skip before returning the results
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_devices(
        &self,
        request: &ListDevicesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Device>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "devices",
                None,
                QueryBuilder::new()
                    .string("sortBy", request.sort_by.clone())
                    .structured_query("query", request.query.clone())
                    .serialize("sortOrder", request.sort_order.clone())
                    .float("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn create_a_device(
        &self,
        request: &PostDevicesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Device, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "devices",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Get details on a specific device.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - Device name or ID. Device names must be URI-encoded if they contain
    /// non-URI-safe characters. If a device is named with another device's ID,
    /// the device with the matching name will be returned.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_a_device(
        &self,
        name_or_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<Device, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("devices/{}", name_or_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a device. Once a device is deleted, it will no longer show up in your list of devices.
    ///
    /// _Before deleting a device, you must delete all associated data._
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - Device name or ID. Device names must be URI-encoded if they contain
    /// non-URI-safe characters. If a device is named with another device's ID,
    /// the device with the matching name will be returned.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_a_device(
        &self,
        name_or_id: &String,
        options: Option<RequestOptions>,
    ) -> Result<DeleteDevicesNameOrIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("devices/{}", name_or_id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn update_a_device(
        &self,
        name_or_id: &String,
        request: &PatchDevicesNameOrIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<Device, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("devices/{}", name_or_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
