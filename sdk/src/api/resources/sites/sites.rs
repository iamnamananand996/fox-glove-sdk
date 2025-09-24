use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SitesClient {
    pub http_client: HttpClient,
}

impl SitesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a list of sites.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_sites(&self, options: Option<RequestOptions>) -> Result<Vec<Site>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "sites", None, None, options)
            .await
    }

    /// Create a new site.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_a_site(
        &self,
        request: &PostSitesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Site, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "sites",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Get details for a specific site.
    ///
    /// # Arguments
    ///
    /// * `id` - Site ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_site_details(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<Site, ApiError> {
        self.http_client
            .execute_request(Method::GET, &format!("sites/{}", id), None, None, options)
            .await
    }

    /// Delete a site.
    ///
    /// _NOTE: Site deletion is permanent and cannot be undone. Any recordings stored at this site
    /// will no longer be available through Foxglove._
    ///
    /// For `edge` and `self-hosted` sites, you should shut down your deployment before deleting
    /// the site through the API.
    ///
    /// If the site type is `self-hosted`, the contents of your inbox and lake buckets will not be
    /// affected by this action, and should be cleaned up separately after deleting the site.
    ///
    /// If the site type is `edge`, any files in edge storage will not be affected by this action.
    ///
    /// # Arguments
    ///
    /// * `id` - Site ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_a_site(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<DeleteSitesIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("sites/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the name or retention period for a Site.
    ///
    /// # Arguments
    ///
    /// * `id` - Site ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_site_details(
        &self,
        id: &String,
        request: &PatchSitesIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<Site, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("sites/{}", id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
