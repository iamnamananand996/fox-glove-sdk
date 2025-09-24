use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EventsClient {
    pub http_client: HttpClient,
}

impl EventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a list of events.
    ///
    /// Use the `query` parameter to filter events on key/value criteria.
    ///
    /// Syntax:
    ///
    /// * `key:value`: matches events with metadata that contains a key named `key` with a value of `value`; use double quotes if the value contains spaces or special characters
    /// * `key:value1,value2`: matches events with metadata that contains a key named `key` with a value of either `value1` or `value2`
    /// * `key:*`: matches events where any metadata that contains a key named `key`
    /// * `*:value`: matches events where any metadata that contains `value` as a value
    /// * `foo`: matches events with metadata where any key or value string contains `foo`
    ///
    /// Multiple qualifiers can be used in the same query string; this will filter events where metadata matches the intersection of the qualifiers (AND).
    ///
    /// Examples:
    ///
    /// * `key1:value1 key2:value2`: matches metadata that contains both a key named `key1` with its value `value1` and another key named `key2` with its value `value2`
    /// * `key:"value with spaces"`: matches metadata with a key named `key` and its value `value with spaces`
    /// * `key:value foo`: matches metadata that contains both a key named `key` with its value `value` and any key or value that contains the text `foo`
    ///
    /// > Note: The `start` and `end` query arguments will find any events which intersect the query range (inclusive of start and end).
    ///
    /// # Arguments
    ///
    /// * `start` - Start of an inclusive time range
    /// * `end` - End of an inclusive time range
    /// * `created_after` - Return all events created after this date and time
    /// * `updated_after` - Return all events updated after this date and time
    /// * `device_id` - Filter events matching device ID
    /// * `device_name` - Name of device associated with the event
    /// * `query` - Event query string. Comprises a space-separated list of event queries, where the syntax of those queries is described above.
    /// * `sort_by` - field to sort response items by
    /// * `sort_order` - Sort order for the `sortBy` field
    /// * `limit` - Maximum number of items to return
    /// * `offset` - Number of items to skip before returning the results
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_events(
        &self,
        request: &ListEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Event>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "events",
                None,
                QueryBuilder::new()
                    .datetime("start", request.start.clone())
                    .datetime("end", request.end.clone())
                    .datetime("createdAfter", request.created_after.clone())
                    .datetime("updatedAfter", request.updated_after.clone())
                    .string("deviceId", request.device_id.clone())
                    .string("deviceName", request.device_name.clone())
                    .structured_query("query", request.query.clone())
                    .serialize("sortBy", request.sort_by.clone())
                    .serialize("sortOrder", request.sort_order.clone())
                    .float("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new event.
    ///
    /// Note: Creating an new event currently requires a device ID or device
    /// name, however the `device` field on the Event resource responses is
    /// optional to allow future API expansion for attaching events to other
    /// types of resources.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_an_event(
        &self,
        request: &PostEventsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Event, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "events",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn get_an_event(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<Event, ApiError> {
        self.http_client
            .execute_request(Method::GET, &format!("events/{}", id), None, None, options)
            .await
    }

    pub async fn delete_an_event(
        &self,
        id: &String,
        options: Option<RequestOptions>,
    ) -> Result<DeleteEventsIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("events/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn update_an_event(
        &self,
        id: &String,
        request: &PatchEventsIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<Event, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("events/{}", id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
