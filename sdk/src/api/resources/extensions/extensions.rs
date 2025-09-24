use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ExtensionsClient {
    pub http_client: HttpClient,
}

impl ExtensionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// **Available on Free, Team, and Enterprise plans**
    ///
    /// Publish a new [Foxglove extension](https://docs.foxglove.dev/docs/visualization/extensions/introduction)
    /// or an updated version of an existing extension. When uploaded, the extension will automatically be
    /// installed for users in your organization.
    ///
    /// See our packaging tool [`create-foxglove-extension`](https://github.com/foxglove/create-foxglove-extension/)
    /// for more information on how to create an extension.
    ///
    /// Read the [docs](https://docs.foxglove.dev/docs/visualization/extensions/introduction)
    /// for more information on Foxglove extensions.
    ///
    /// # Arguments
    ///
    /// * `request` - The request headers must contain `Content-Type: application/octet-stream`.
    /// The request body must contain the contents of your (`.foxe`) extension file.
    /// All other information about the extension will be parsed from the package.json file.
    ///
    /// **`package.json` requirements**
    ///
    /// An extension is uniquely identified by its publisher and name, which are
    /// both required in your `package.json`. Both values are case-insensitive,
    /// so a package named `custompanel` would be the same as one named `CustomPanel`.
    ///
    /// To update an extension, change its version in `package.json`, create a
    /// new `.foxe` bundle, and upload it via this endpoint. Version numbers must
    /// be unique to each extension. There are no other restrictions on versioning schemes.
    ///
    /// To publish an extension to your organization, your `package.json` must define
    /// a `displayName`. This is displayed to users in your organization.
    ///
    /// Once an extension is uploaded, its `displayName` may not be changed by future versions.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn publish_an_extension(
        &self,
        request: &Vec<u8>,
        options: Option<RequestOptions>,
    ) -> Result<PostExtensionUploadResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "extension-upload",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Organization admins can share and manage Foxglove extensions. Check out the
    /// [docs](https://docs.foxglove.dev/docs/visualization/extensions/introduction) to learn more.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_extensions(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Extension>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "extensions", None, None, options)
            .await
    }

    pub async fn get_an_extension(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<ExtensionWithSignedLink, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("extensions/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Once deleted, the extension will no longer be available within your organization.
    ///
    /// # Arguments
    ///
    /// * `id` - Extension ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_an_extension(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<DeleteExtensionsIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("extensions/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
