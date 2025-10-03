# Reference
## Devices
<details><summary><code>client.devices.<a href="/src/api/resources/devices/client.rs">list_devices</a>(sort_by: Option<Option<String>>, query: Option<Option<String>>, sort_order: Option<Option<GetDevicesRequestSortOrder>>, limit: Option<Option<f64>>, offset: Option<Option<i64>>) -> Result<Vec<Device>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of devices.

**Filtering by custom properties**

Use the `query` parameter to filter devices on custom properties.
Syntax:
* `properties.key:value`: matches devices with a property that contains a key named `key` with a value of `value`; use double quotes if the value contains spaces or special characters
* `properties.key:value1,value2`: matches devices with a property that contains a key named `key` and its value is either `value1` or `value2`
* `properties.key:*`: matches devices with a property that contains a key named `key` and any value
* `foo`: matches devices with properties where any key or stringified value contains `foo`
Multiple qualifiers can be used in the same query string; this will filter devices matching the intersection of the qualifiers (AND).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .devices
        .list_devices(
            &ListDevicesQueryRequest {
                sort_by: Some("sortBy".to_string()),
                query: Some("query".to_string()),
                sort_order: Some(GetDevicesRequestSortOrder::Asc),
                limit: Some(1.1),
                offset: Some(1),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sort_by:** `Option<String>` — Field to sort items by ("id", "name", or a custom property key prefixed with `properties.`)
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Space-separated query string for device custom properties. Each custom property key must be valid and prefixed with "properties.". See above for syntax and examples.
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<GetDevicesRequestSortOrder>` — Sort order for the `sortBy` field
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<f64>` — Maximum number of items to return
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Number of items to skip before returning the results
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.devices.<a href="/src/api/resources/devices/client.rs">create_a_device</a>(request: PostDevicesRequest) -> Result<Device, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .devices
        .create_a_device(
            &PostDevicesRequest {
                name: DeviceName("name".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `DeviceName` 
    
</dd>
</dl>

<dl>
<dd>

**properties:** `Option<std::collections::HashMap<String, PostDevicesRequestPropertiesValue>>` 

A key-value map, where each key is one of your pre-defined device custom property keys.
Keys which are not recognized as custom properties will be ignored.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.devices.<a href="/src/api/resources/devices/client.rs">get_a_device</a>(name_or_id: String) -> Result<Device, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get details on a specific device.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .devices
        .get_a_device(&"nameOrId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name_or_id:** `String` 

Device name or ID. Device names must be URI-encoded if they contain
non-URI-safe characters. If a device is named with another device's ID,
the device with the matching name will be returned.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.devices.<a href="/src/api/resources/devices/client.rs">delete_a_device</a>(name_or_id: String) -> Result<DeleteDevicesNameOrIdResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a device. Once a device is deleted, it will no longer show up in your list of devices.

_Before deleting a device, you must delete all associated data._
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .devices
        .delete_a_device(&"nameOrId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name_or_id:** `String` 

Device name or ID. Device names must be URI-encoded if they contain
non-URI-safe characters. If a device is named with another device's ID,
the device with the matching name will be returned.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.devices.<a href="/src/api/resources/devices/client.rs">update_a_device</a>(name_or_id: String, request: PatchDevicesNameOrIdRequest) -> Result<Device, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .devices
        .update_a_device(
            &"nameOrId".to_string(),
            &PatchDevicesNameOrIdRequest {},
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name_or_id:** `String` 

Device name or ID. Device names must be URI-encoded if they contain
non-URI-safe characters. If a device is named with another device's ID,
the device with the matching name will be returned.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Device names must be unique in your organization.
    
</dd>
</dl>

<dl>
<dd>

**retain_recordings_seconds:** `Option<f64>` 

Optionally set a retention period for recordings created on a device running the
Foxglove Agent. If set to zero, recordings are retained indefinitely. This is
only relevant for devices that have an Agent installed.
    
</dd>
</dl>

<dl>
<dd>

**properties:** `Option<std::collections::HashMap<String, serde_json::Value>>` 

A key-value map, where each key is one of your pre-defined device custom property keys.
Keys which are not recognized as custom properties will be ignored.
Keys which are not included in the request, but exist on the device, will be unchanged.
To unset a property, pass `null` as the value.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Recordings
<details><summary><code>client.recordings.<a href="/src/api/resources/recordings/client.rs">list_recordings</a>(start: Option<Option<String>>, end: Option<Option<String>>, path: Option<Option<String>>, site_id: Option<Option<String>>, edge_site_id: Option<Option<String>>, device_id: Option<Option<String>>, device_name: Option<Option<String>>, topic: Option<Option<String>>, import_status: Option<Option<GetRecordingsRequestImportStatus>>, limit: Option<Option<f64>>, offset: Option<Option<i64>>, sort_by: Option<Option<GetRecordingsRequestSortBy>>, sort_order: Option<Option<GetRecordingsRequestSortOrder>>) -> Result<Vec<Recording>, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chrono::{DateTime, Utc};
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .recordings
        .list_recordings(
            &ListRecordingsQueryRequest {
                start: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                end: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                path: Some("path".to_string()),
                site_id: Some("site.id".to_string()),
                edge_site_id: Some("edgeSite.id".to_string()),
                device_id: Some("deviceId".to_string()),
                device_name: Some("deviceName".to_string()),
                topic: Some("topic".to_string()),
                import_status: Some(GetRecordingsRequestImportStatus::None),
                limit: Some(1.1),
                offset: Some(1),
                sort_by: Some(GetRecordingsRequestSortBy::DeviceName),
                sort_order: Some(GetRecordingsRequestSortOrder::Asc),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**start:** `Option<String>` — Start of an inclusive time range
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<String>` — End of an inclusive time range
    
</dd>
</dl>

<dl>
<dd>

**path:** `Option<String>` — Filter response to recordings with this path
    
</dd>
</dl>

<dl>
<dd>

**site_id:** `Option<String>` — Filter response to recordings stored at the Primary Site with this ID
    
</dd>
</dl>

<dl>
<dd>

**edge_site_id:** `Option<String>` — Filter response to recordings stored at the Edge Site with this ID
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `Option<String>` — Filter response to recordings for the device with this ID, empty string for those without any
    
</dd>
</dl>

<dl>
<dd>

**device_name:** `Option<String>` — Filter response to recordings for the device with this name
    
</dd>
</dl>

<dl>
<dd>

**topic:** `Option<String>` — Filter response to recordings containing the topic
    
</dd>
</dl>

<dl>
<dd>

**import_status:** `Option<GetRecordingsRequestImportStatus>` — Filter response to recordings with this import status
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<f64>` — Maximum number of items to return
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Number of items to skip before returning the results
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<GetRecordingsRequestSortBy>` 

Sort returned recordings by a field in the response type. Specifying `duration` sorts by
the duration between the recording `start` and `end` fields.
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<GetRecordingsRequestSortOrder>` — Sort order for the `sortBy` field
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.recordings.<a href="/src/api/resources/recordings/client.rs">get_a_recording</a>(key_or_id: String) -> Result<Recording, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get details on a specific recording.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .recordings
        .get_a_recording(&"keyOrId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**key_or_id:** `String` — Recording Key or ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.recordings.<a href="/src/api/resources/recordings/client.rs">delete_a_recording</a>(key_or_id: String) -> Result<DeleteRecordingsKeyOrIdResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a recording. Deleting a recording also deletes the data for that recording
(including attachments, messages, metadata, etc).

Note: For recordings stored at an Edge Site, this method deletes only
the imported data for that recording, leaving the edge copy intact.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .recordings
        .delete_a_recording(&"keyOrId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**key_or_id:** `String` — Recording Key or ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.recordings.<a href="/src/api/resources/recordings/client.rs">import_from_edge</a>(key_or_id: String) -> Result<PostRecordingsKeyOrIdImportResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Request import of a recording from an Edge Site to a Primary Site. Importing a recording
makes the data (messages, metadata, attachments, etc.) available for download and streaming.

If the recording is successfully queued for import, is already imported, or already queued for
import, this endpoint will return a 200 response and include the recording ID and the
`importStatus`.

An import status of `complete` indicates the recording is already imported. Poll the `GET
v1/recordings/{id}` endpoint to observe changes to the `importStatus`.

If the recording cannot be found or is unavailable for import because the edge copy or site
is deleted, this endpoint will return a 404 response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .recordings
        .import_from_edge(&"keyOrId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**key_or_id:** `String` — Recording ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.recordings.<a href="/src/api/resources/recordings/client.rs">list_pending_imports</a>(request_id: Option<Option<String>>, key: Option<Option<String>>, device_id: Option<Option<String>>, device_name: Option<Option<String>>, error: Option<Option<String>>, filename: Option<Option<String>>, updated_since: Option<Option<String>>, show_completed: Option<Option<bool>>, show_quarantined: Option<Option<bool>>, site_id: Option<Option<String>>, sort_by: Option<Option<GetDataPendingImportsRequestSortBy>>, sort_order: Option<Option<GetDataPendingImportsRequestSortOrder>>, limit: Option<Option<f64>>, offset: Option<Option<i64>>) -> Result<Vec<PendingImport>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

List the pending imports. These are in-progress import jobs for newly uploaded recordings.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chrono::{DateTime, Utc};
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .recordings
        .list_pending_imports(
            &ListPendingImportsQueryRequest {
                request_id: Some("requestId".to_string()),
                key: Some("key".to_string()),
                device_id: Some("deviceId".to_string()),
                device_name: Some("deviceName".to_string()),
                error: Some("error".to_string()),
                filename: Some("filename".to_string()),
                updated_since: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                show_completed: Some(true),
                show_quarantined: Some(true),
                site_id: Some("siteId".to_string()),
                sort_by: Some(GetDataPendingImportsRequestSortBy::CreatedAt),
                sort_order: Some(GetDataPendingImportsRequestSortOrder::Asc),
                limit: Some(1.1),
                offset: Some(1),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**request_id:** `Option<String>` — A specific import request ID
    
</dd>
</dl>

<dl>
<dd>

**key:** `Option<String>` — The unique key optionally provided when importing
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `Option<String>` — ID of device associated with the pending import
    
</dd>
</dl>

<dl>
<dd>

**device_name:** `Option<String>` — Name of device associated with the pending import
    
</dd>
</dl>

<dl>
<dd>

**error:** `Option<String>` — A string to filter based on error messages
    
</dd>
</dl>

<dl>
<dd>

**filename:** `Option<String>` — Filename to exactly match
    
</dd>
</dl>

<dl>
<dd>

**updated_since:** `Option<String>` — Filter pending imports updated since this time
    
</dd>
</dl>

<dl>
<dd>

**show_completed:** `Option<bool>` — Include completed requests
    
</dd>
</dl>

<dl>
<dd>

**show_quarantined:** `Option<bool>` — Include quarantined requests
    
</dd>
</dl>

<dl>
<dd>

**site_id:** `Option<String>` — Filter response to imports at site with this ID
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<GetDataPendingImportsRequestSortBy>` — Sort by a single field of the import type
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<GetDataPendingImportsRequestSortOrder>` — Sort order for the `sortBy` field
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<f64>` — Maximum number of items to return
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Number of items to skip before returning the results
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.recordings.<a href="/src/api/resources/recordings/client.rs">upload_a_recording</a>(request: PostDataUploadRequest) -> Result<PostDataUploadResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to upload data to your `foxglove-hosted` site. The upload is a two-request
process.

1. Make a request to this upload endpoint to create an upload `link`.
2. Issue a PUT HTTP request to the `link` response field URL.

_Your PUT request header should have `Content-Type: application/octet-stream`, and your
request body should contain your file content._

Note: If you are using a self-hosted site, see [this
guide](https://docs.foxglove.dev/docs/primary-sites/self-hosting/manage-data) for uploading data.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .recordings
        .upload_a_recording(
            &PostDataUploadRequest {
                filename: "filename".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**device_id:** `Option<String>` — Foxglove ID of the associated device
    
</dd>
</dl>

<dl>
<dd>

**device_name:** `Option<String>` — Unique name of the associated device. If no device exists with this name, the device will be created.
    
</dd>
</dl>

<dl>
<dd>

**filename:** `String` — Name of the file that will be uploaded
    
</dd>
</dl>

<dl>
<dd>

**key:** `Option<String>` — A unique key to identify the recording
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Imports
<details><summary><code>client.imports.<a href="/src/api/resources/imports/client.rs">list_imports</a>(device_id: Option<Option<String>>, filename: Option<Option<String>>, start: Option<Option<String>>, end: Option<Option<String>>, data_start: Option<Option<String>>, data_end: Option<Option<String>>, sort_by: Option<Option<GetDataImportsRequestSortBy>>, sort_order: Option<Option<GetDataImportsRequestSortOrder>>, limit: Option<Option<f64>>, offset: Option<Option<i64>>) -> Result<Vec<Import>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**This endpoint is deprecated. Use the [list recordings](#tag/Recordings/paths/~1recordings/get) endpoint instead.**
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chrono::{DateTime, Utc};
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .imports
        .list_imports(
            &ListImportsQueryRequest {
                device_id: Some("deviceId".to_string()),
                filename: Some("filename".to_string()),
                start: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                end: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                data_start: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                data_end: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                sort_by: Some(GetDataImportsRequestSortBy::ImportId),
                sort_order: Some(GetDataImportsRequestSortOrder::Asc),
                limit: Some(1.1),
                offset: Some(1),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**device_id:** `Option<String>` — ID of device associated with the exported data
    
</dd>
</dl>

<dl>
<dd>

**filename:** `Option<String>` — Filename to match
    
</dd>
</dl>

<dl>
<dd>

**start:** `Option<String>` — Start of an inclusive time range
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<String>` — End of an inclusive time range
    
</dd>
</dl>

<dl>
<dd>

**data_start:** `Option<String>` — Inclusive start of message log time
    
</dd>
</dl>

<dl>
<dd>

**data_end:** `Option<String>` — Inclusive end of message log time
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<GetDataImportsRequestSortBy>` — Sort by a single field of the import type
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<GetDataImportsRequestSortOrder>` — Sort order for the `sortBy` field
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<f64>` — Maximum number of items to return
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Number of items to skip before returning the results
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.imports.<a href="/src/api/resources/imports/client.rs">delete_multiple_imports</a>() -> Result<DeleteDataImportsResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes multiple imports by ID. Returns an array of result objects, which indicate whether a given import was successfully deleted. An import that has already been deleted will result in "notFound".
_Note: All imports must belong to the same site. If any import belongs to a different site, the entire request is rejected with a 400 response._
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .imports
        .delete_multiple_imports(&DeleteMultipleImportsQueryRequest {}, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `Option<String>` — ID of import to delete. You can specify up to 50 IDs (for example, `?id=abc&id=def&...`).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.imports.<a href="/src/api/resources/imports/client.rs">delete_an_import</a>(import_id: String, device_id: Option<Option<String>>) -> Result<DeleteDataImportsImportIdResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**This endpoint is deprecated. Use the [delete recording](#tag/Recordings/paths/~1recordings~1%7Bid%7D/delete) endpoint instead.**

Deleting an import deletes all data associated with the import.

**This action is permanent and cannot be undone.**
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .imports
        .delete_an_import(
            &"importId".to_string(),
            &DeleteAnImportQueryRequest {
                device_id: Some("deviceId".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**import_id:** `String` — The `importId` of an import
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `Option<String>` — The deviceId from the import record
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## RecordingAttachments
<details><summary><code>client.recording_attachments.<a href="/src/api/resources/recording_attachments/client.rs">list_attachments</a>(recording_id: Option<Option<String>>, site_id: Option<Option<String>>, device_id: Option<Option<String>>, device_name: Option<Option<String>>, sort_by: Option<Option<String>>, sort_order: Option<Option<GetRecordingAttachmentsRequestSortOrder>>, limit: Option<Option<f64>>, offset: Option<Option<i64>>) -> Result<Vec<RecordingAttachment>, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .recording_attachments
        .list_attachments(
            &ListAttachmentsQueryRequest {
                recording_id: Some("recordingId".to_string()),
                site_id: Some("siteId".to_string()),
                device_id: Some("deviceId".to_string()),
                device_name: Some("deviceName".to_string()),
                sort_by: Some("logTime".to_string()),
                sort_order: Some(GetRecordingAttachmentsRequestSortOrder::Asc),
                limit: Some(1.1),
                offset: Some(1),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**recording_id:** `Option<String>` — filter by recording ID
    
</dd>
</dl>

<dl>
<dd>

**site_id:** `Option<String>` — filter by Primary Site ID
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `Option<String>` — filter by device ID
    
</dd>
</dl>

<dl>
<dd>

**device_name:** `Option<String>` — filter by device name
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — Sort by a single field of the attachment
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<GetRecordingAttachmentsRequestSortOrder>` — Sort order for the `sortBy` field
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<f64>` — Maximum number of items to return
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Number of items to skip before returning the results
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.recording_attachments.<a href="/src/api/resources/recording_attachments/client.rs">get_an_attachment</a>(id: String) -> Result<RecordingAttachment, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .recording_attachments
        .get_an_attachment(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the attachment
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.recording_attachments.<a href="/src/api/resources/recording_attachments/client.rs">download_an_attachment</a>(id: String) -> Result<(), ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

To download an attachment make a request to this endpoint and follow the 302 redirect. The
attachment will download directly from the Primary Site.

Note: The redirect link expires after 15 minutes.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .recording_attachments
        .download_an_attachment(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the attachment to download
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Coverage
<details><summary><code>client.coverage.<a href="/src/api/resources/coverage/client.rs">list_coverage</a>(start: Option<Option<String>>, end: Option<Option<String>>, tolerance: Option<Option<f64>>, device_id: Option<Option<String>>, device_name: Option<Option<String>>, include_edge_recordings: Option<Option<bool>>, import_id: Option<Option<String>>, recording_id: Option<Option<String>>, recording_key: Option<Option<String>>) -> Result<Vec<Coverage>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

A coverage range represents a time span for which Foxglove has data for a
given device.

Your must specify the `start` and `end` arguments when making a coverage request.

Note: By default, only coverage ranges with imported recordings are returned. To include
coverage ranges with unimported recordings from an Edge Site or Agent, set the
`includeEdgeRecordings` query parameter to true
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chrono::{DateTime, Utc};
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .coverage
        .list_coverage(
            &ListCoverageQueryRequest {
                start: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                end: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                tolerance: Some(1.1),
                device_id: Some("deviceId".to_string()),
                device_name: Some("deviceName".to_string()),
                include_edge_recordings: Some(true),
                import_id: Some("importId".to_string()),
                recording_id: Some("recordingId".to_string()),
                recording_key: Some("recordingKey".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**start:** `Option<String>` — Start of an inclusive time range
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<String>` — End of an inclusive time range
    
</dd>
</dl>

<dl>
<dd>

**tolerance:** `Option<f64>` 

Minimum interval (in seconds) that ranges must be separated by to be considered discrete.
Currently, the minimum meaningful value is 14s and smaller values will be clamped to this value.
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `Option<String>` — Filter coverage by device ID
    
</dd>
</dl>

<dl>
<dd>

**device_name:** `Option<String>` — Name of device associated with the data
    
</dd>
</dl>

<dl>
<dd>

**include_edge_recordings:** `Option<bool>` 

Include recordings from an Edge Site or Agent in the response.

When edge recordings are included, each item in the response array will also include the
`importStatus` for the coverage range.
    
</dd>
</dl>

<dl>
<dd>

**import_id:** `Option<String>` — Filter coverage by import ID
    
</dd>
</dl>

<dl>
<dd>

**recording_id:** `Option<String>` — Filter coverage by recording ID
    
</dd>
</dl>

<dl>
<dd>

**recording_key:** `Option<String>` — Filter coverage by recordingKey
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## StreamData
<details><summary><code>client.stream_data.<a href="/src/api/resources/stream_data/client.rs">download_data</a>(request: PostDataStreamRequest) -> Result<PostDataStreamResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint returns a `link` URL where you can download your data as an `.mcap` or `.bag`
file.

To download your data:
  1. Make a request to this endpoint.
  2. Make a `GET` request to the `link` URL.

One of `recordingId`, `key`, `importId` (deprecated) or all three of
`deviceId`/`deviceName`, `start`, and `end` must be specified.

_Note: You can only export a `.bag` file if you originally uploaded a `.bag` file._
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .stream_data
        .download_data(&PostDataStreamRequest {}, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**topics:** `Option<Vec<String>>` — List of topics to include in the exported file (defaults to all topics)
    
</dd>
</dl>

<dl>
<dd>

**output_format:** `Option<PostDataStreamRequestOutputFormat>` 

Output file format.
  * `bag1` - output a .bag file
  * `mcap` - output a .mcap file
  * `mcap0` - Deprecated. Use `mcap`.
    
</dd>
</dl>

<dl>
<dd>

**compression_format:** `Option<PostDataStreamRequestCompressionFormat>` 

Output compression format for chunks. Only valid if `outputFormat` is `mcap`.
  * `""` - no compression
  * `zstd` - zstd compression
  * `lz4` - LZ4 compression (default)
    
</dd>
</dl>

<dl>
<dd>

**include_attachments:** `Option<bool>` 

Include attachments in streamed data. One of `recordingId` or `importId`
(deprecated) must also be set. Only valid for mcap outputFormat.
    
</dd>
</dl>

<dl>
<dd>

**is_hosted:** `Option<bool>` — true if the import is hosted
    
</dd>
</dl>

<dl>
<dd>

**replay_policy:** `Option<PostDataStreamRequestReplayPolicy>` 

If set to "lastPerChannel", then the stream will include the most recent message
on each channel, even if it comes before the requested `start`, as long as it is
within the window of `replayLookbackSeconds` seconds before `start`.

The default, `""` (no policy), means no messages before `start` are included.
    
</dd>
</dl>

<dl>
<dd>

**replay_lookback_seconds:** `Option<f64>` 

The maximum amount of time (in seconds) to look back before `start` in order to
find the latest message. Only used if `replayPolicy` is set to "lastPerChannel".
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `Option<String>` — ID of device associated with the exported data
    
</dd>
</dl>

<dl>
<dd>

**device_name:** `Option<String>` — Name of device associated with the exported data.
    
</dd>
</dl>

<dl>
<dd>

**start:** `Option<String>` — Inclusive start of requested time range. If start is provided, end must be too.
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<String>` — Inclusive end of requested time range. If end is provided, start must be too.
    
</dd>
</dl>

<dl>
<dd>

**import_id:** `Option<String>` — ID of the import to stream
    
</dd>
</dl>

<dl>
<dd>

**recording_id:** `Option<String>` — ID of the recording to stream
    
</dd>
</dl>

<dl>
<dd>

**recording_key:** `Option<String>` — Key of recording to stream
    
</dd>
</dl>

<dl>
<dd>

**key:** `Option<String>` — Key of recording to stream
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Topics
<details><summary><code>client.topics.<a href="/src/api/resources/topics/client.rs">list_topics</a>(start: Option<Option<String>>, end: Option<Option<String>>, import_id: Option<Option<String>>, recording_id: Option<Option<String>>, recording_key: Option<Option<String>>, device_id: Option<Option<String>>, device_name: Option<Option<String>>, include_schemas: Option<Option<bool>>, sort_by: Option<Option<GetDataTopicsRequestSortBy>>, sort_order: Option<Option<GetDataTopicsRequestSortOrder>>, limit: Option<Option<f64>>, offset: Option<Option<i64>>) -> Result<Vec<Topic>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of topics available for a device within a given time range.

By default, this endpoint will not return the `schema` for each topic. To include
the schemas, you must provide the `includeSchemas` query parameter.

Use `start` and `end` to limit the response to overlapping recording ranges.

Topics for not-imported recordings are only returned if no parameter is provided besides
recordingId or recordingKey. This is because most parameters need the imported files to
filter, and can only return an empty list if imports are unavailable.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chrono::{DateTime, Utc};
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .topics
        .list_topics(
            &ListTopicsQueryRequest {
                start: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                end: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                import_id: Some("importId".to_string()),
                recording_id: Some("recordingId".to_string()),
                recording_key: Some("recordingKey".to_string()),
                device_id: Some("deviceId".to_string()),
                device_name: Some("deviceName".to_string()),
                include_schemas: Some(true),
                sort_by: Some(GetDataTopicsRequestSortBy::Topic),
                sort_order: Some(GetDataTopicsRequestSortOrder::Asc),
                limit: Some(1.1),
                offset: Some(1),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**start:** `Option<String>` — Start of an inclusive time range
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<String>` — End of an inclusive time range
    
</dd>
</dl>

<dl>
<dd>

**import_id:** `Option<String>` — ID of the import from which to list topics
    
</dd>
</dl>

<dl>
<dd>

**recording_id:** `Option<String>` — ID of the recording from which to list topics
    
</dd>
</dl>

<dl>
<dd>

**recording_key:** `Option<String>` — Key of the recording from which to list topics
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `Option<String>` — ID of device being queried
    
</dd>
</dl>

<dl>
<dd>

**device_name:** `Option<String>` — Name of device being queried
    
</dd>
</dl>

<dl>
<dd>

**include_schemas:** `Option<bool>` — Whether full schemas should be included in the response
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<GetDataTopicsRequestSortBy>` — Sort by a single field of the topic type ("topic" or "version")
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<GetDataTopicsRequestSortOrder>` — Sort order for the `sortBy` field
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<f64>` — Maximum number of items to return
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Number of items to skip before returning the results
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Extensions
<details><summary><code>client.extensions.<a href="/src/api/resources/extensions/client.rs">list_extensions</a>() -> Result<Vec<Extension>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Organization admins can share and manage Foxglove extensions. Check out the
[docs](https://docs.foxglove.dev/docs/visualization/extensions/introduction) to learn more.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client.extensions.list_extensions(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.extensions.<a href="/src/api/resources/extensions/client.rs">get_an_extension</a>(id: String) -> Result<ExtensionWithSignedLink, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .extensions
        .get_an_extension(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Extension ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.extensions.<a href="/src/api/resources/extensions/client.rs">delete_an_extension</a>(id: String) -> Result<DeleteExtensionsIdResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Once deleted, the extension will no longer be available within your organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .extensions
        .delete_an_extension(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Extension ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Events
<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">list_events</a>(start: Option<Option<String>>, end: Option<Option<String>>, created_after: Option<Option<String>>, updated_after: Option<Option<String>>, device_id: Option<Option<String>>, device_name: Option<Option<String>>, query: Option<Option<String>>, sort_by: Option<Option<GetEventsRequestSortBy>>, sort_order: Option<Option<GetEventsRequestSortOrder>>, limit: Option<Option<f64>>, offset: Option<Option<i64>>) -> Result<Vec<Event>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of events.

Use the `query` parameter to filter events on key/value criteria.

Syntax:

* `key:value`: matches events with metadata that contains a key named `key` with a value of `value`; use double quotes if the value contains spaces or special characters
* `key:value1,value2`: matches events with metadata that contains a key named `key` with a value of either `value1` or `value2`
* `key:*`: matches events where any metadata that contains a key named `key`
* `*:value`: matches events where any metadata that contains `value` as a value
* `foo`: matches events with metadata where any key or value string contains `foo`

Multiple qualifiers can be used in the same query string; this will filter events where metadata matches the intersection of the qualifiers (AND).

Examples:

* `key1:value1 key2:value2`: matches metadata that contains both a key named `key1` with its value `value1` and another key named `key2` with its value `value2`
* `key:"value with spaces"`: matches metadata with a key named `key` and its value `value with spaces`
* `key:value foo`: matches metadata that contains both a key named `key` with its value `value` and any key or value that contains the text `foo`

> Note: The `start` and `end` query arguments will find any events which intersect the query range (inclusive of start and end).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chrono::{DateTime, Utc};
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .events
        .list_events(
            &ListEventsQueryRequest {
                start: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                end: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                created_after: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                updated_after: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                device_id: Some("deviceId".to_string()),
                device_name: Some("deviceName".to_string()),
                query: Some("query".to_string()),
                sort_by: Some(GetEventsRequestSortBy::Id),
                sort_order: Some(GetEventsRequestSortOrder::Asc),
                limit: Some(1.1),
                offset: Some(1),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**start:** `Option<String>` — Start of an inclusive time range
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<String>` — End of an inclusive time range
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Return all events created after this date and time
    
</dd>
</dl>

<dl>
<dd>

**updated_after:** `Option<String>` — Return all events updated after this date and time
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `Option<String>` — Filter events matching device ID
    
</dd>
</dl>

<dl>
<dd>

**device_name:** `Option<String>` — Name of device associated with the event
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Event query string. Comprises a space-separated list of event queries, where the syntax of those queries is described above.
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<GetEventsRequestSortBy>` — field to sort response items by
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<GetEventsRequestSortOrder>` — Sort order for the `sortBy` field
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<f64>` — Maximum number of items to return
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Number of items to skip before returning the results
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">create_an_event</a>(request: PostEventsRequest) -> Result<Event, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new event.

Note: Creating an new event currently requires a device ID or device
  name, however the `device` field on the Event resource responses is
  optional to allow future API expansion for attaching events to other
  types of resources.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chrono::{DateTime, Utc};
use foxglove::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .events
        .create_an_event(
            &PostEventsRequest {
                start: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
                end: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**device_id:** `Option<String>` — ID of the device to associate with the event
    
</dd>
</dl>

<dl>
<dd>

**device_name:** `Option<String>` — Name of the device to associate with the event
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<std::collections::HashMap<String, String>>` — An object with user-defined string keys and string values; key order is not preserved
    
</dd>
</dl>

<dl>
<dd>

**start:** `String` — Event start time (inclusive)
    
</dd>
</dl>

<dl>
<dd>

**end:** `String` — Event end time (inclusive)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">get_an_event</a>(id: String) -> Result<Event, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client.events.get_an_event(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the event
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">delete_an_event</a>(id: String) -> Result<DeleteEventsIdResponse, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client.events.delete_an_event(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the event
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">update_an_event</a>(id: String, request: PatchEventsIdRequest) -> Result<Event, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .events
        .update_an_event(&"id".to_string(), &PatchEventsIdRequest {}, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the event
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<std::collections::HashMap<String, String>>` — An object with user-defined string keys and string values; key order is not preserved
    
</dd>
</dl>

<dl>
<dd>

**start:** `Option<String>` — Event start time (inclusive)
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<String>` — Event end time (inclusive)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## LakeFiles
<details><summary><code>client.lake_files.<a href="/src/api/resources/lake_files/client.rs">list_lake_files</a>(site_id: Option<String>, device_id: Option<Option<String>>, device_name: Option<Option<String>>, recording_id: Option<Option<String>>, recording_key: Option<Option<String>>, start: Option<Option<String>>, end: Option<Option<String>>, topic: Option<Option<String>>) -> Result<Vec<GetLakeFilesResponseItem>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint returns a list of MCAP files in the lake bucket for a given Primary Site.

For each recording that has been imported, multiple files are created — one per topic.

This endpoint is only supported for self-managed sites.

A query must be limited to a device or recording using one of the following parameters:
- deviceId
- deviceName
- recordingId
- recordingKey

If querying by a device (ID or name), you must also provide `start` and `end` parameters to limit the range of files included.

The range expressed by `start` and `end` must not exceed 24h.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chrono::{DateTime, Utc};
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .lake_files
        .list_lake_files(
            &ListLakeFilesQueryRequest {
                site_id: "siteId".to_string(),
                device_id: Some("deviceId".to_string()),
                device_name: Some("deviceName".to_string()),
                recording_id: Some("recordingId".to_string()),
                recording_key: Some("recordingKey".to_string()),
                start: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                end: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                topic: Some("topic".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**site_id:** `String` — The ID of a self-managed Primary Site for which the files have been imported
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `Option<String>` — ID of the device associated with the imported files
    
</dd>
</dl>

<dl>
<dd>

**device_name:** `Option<String>` — Name of the device associated with the imported files
    
</dd>
</dl>

<dl>
<dd>

**recording_id:** `Option<String>` — A recording ID for which the files have been imported
    
</dd>
</dl>

<dl>
<dd>

**recording_key:** `Option<String>` — A recording key for which the files have been imported
    
</dd>
</dl>

<dl>
<dd>

**start:** `Option<String>` — Inclusive start of an imported recording's time range. If start is provided, end must be too.
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<String>` — Inclusive end of an imported recording's time range. If end is provided, start must be too.
    
</dd>
</dl>

<dl>
<dd>

**topic:** `Option<String>` — Include only imported files matching this topic name
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Layouts
<details><summary><code>client.layouts.<a href="/src/api/resources/layouts/client.rs">list_layouts</a>(updated_since: Option<Option<String>>, include_data: Option<Option<bool>>) -> Result<Vec<Layout>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

List the org layouts.

Note: Only layouts shared with the org are returned in the response; no personal layouts are
returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chrono::{DateTime, Utc};
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .layouts
        .list_layouts(
            &ListLayoutsQueryRequest {
                updated_since: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                include_data: Some(true),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**updated_since:** `Option<String>` — Return only layouts updated since this time.
    
</dd>
</dl>

<dl>
<dd>

**include_data:** `Option<bool>` 

When set to false, the `data` field is omitted from the response items.
This can be used to limit bandwidth when querying many Layouts.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CustomProperties
<details><summary><code>client.custom_properties.<a href="/src/api/resources/custom_properties/client.rs">list_custom_properties</a>(resource_type: Option<Option<String>>) -> Result<Vec<CustomProperty>, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .custom_properties
        .list_custom_properties(
            &ListCustomPropertiesQueryRequest {
                resource_type: Some("device".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**resource_type:** `Option<String>` — Filter properties matching a resource type
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.custom_properties.<a href="/src/api/resources/custom_properties/client.rs">create_a_custom_property</a>(request: NewCustomProperty) -> Result<CustomProperty, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .custom_properties
        .create_a_custom_property(
            &NewCustomProperty {
                key: "key".to_string(),
                label: "label".to_string(),
                resource_type: "device".to_string(),
                value_type: NewCustomPropertyValueType::String,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.custom_properties.<a href="/src/api/resources/custom_properties/client.rs">get_a_custom_property</a>(id: String) -> Result<CustomProperty, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .custom_properties
        .get_a_custom_property(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Custom Property ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.custom_properties.<a href="/src/api/resources/custom_properties/client.rs">delete_a_custom_property</a>(id: String) -> Result<DeleteCustomPropertiesIdResponse, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .custom_properties
        .delete_a_custom_property(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Custom Property ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.custom_properties.<a href="/src/api/resources/custom_properties/client.rs">edit_a_custom_property</a>(id: String, request: PatchCustomPropertiesIdRequest) -> Result<CustomProperty, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .custom_properties
        .edit_a_custom_property(&"id".to_string(), &PatchCustomPropertiesIdRequest {}, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Custom Property ID
    
</dd>
</dl>

<dl>
<dd>

**label:** `Option<String>` — Display label for user interfaces
    
</dd>
</dl>

<dl>
<dd>

**values:** `Option<Vec<String>>` — Reorder or add to enum values. Must be a superset of existing values.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Sites
<details><summary><code>client.sites.<a href="/src/api/resources/sites/client.rs">list_sites</a>() -> Result<Vec<Site>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of sites.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client.sites.list_sites(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sites.<a href="/src/api/resources/sites/client.rs">create_a_site</a>(request: PostSitesRequest) -> Result<Site, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new site.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .sites
        .create_a_site(
            &PostSitesRequest {
                name: "name".to_string(),
                r#type: PostSitesRequestType::SelfHosted,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` — A name for this site
    
</dd>
</dl>

<dl>
<dd>

**type:** `PostSitesRequestType` — The type of site to create.
    
</dd>
</dl>

<dl>
<dd>

**retain_recordings_seconds:** `Option<f64>` 

Optionally set a retention period for recordings created at
this site. If set to zero, recordings are retained indefinitely.
(only available on Edge Sites)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sites.<a href="/src/api/resources/sites/client.rs">get_site_details</a>(id: String) -> Result<Site, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get details for a specific site.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client.sites.get_site_details(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Site ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sites.<a href="/src/api/resources/sites/client.rs">delete_a_site</a>(id: String) -> Result<DeleteSitesIdResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a site.

_NOTE: Site deletion is permanent and cannot be undone. Any recordings stored at this site
will no longer be available through Foxglove._

For `edge` and `self-hosted` sites, you should shut down your deployment before deleting
the site through the API.

If the site type is `self-hosted`, the contents of your inbox and lake buckets will not be
affected by this action, and should be cleaned up separately after deleting the site.

If the site type is `edge`, any files in edge storage will not be affected by this action.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client.sites.delete_a_site(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Site ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sites.<a href="/src/api/resources/sites/client.rs">update_site_details</a>(id: String, request: PatchSitesIdRequest) -> Result<Site, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the name or retention period for a Site.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .sites
        .update_site_details(&"id".to_string(), &PatchSitesIdRequest {}, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Site ID
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — A name for this site
    
</dd>
</dl>

<dl>
<dd>

**retain_recordings_seconds:** `Option<f64>` 

Optionally set a retention period for recordings created at this Edge Site.
If set to zero, recordings are retained indefinitely.
(only available on Edge Sites)
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` 

The URL a self-hosted Primary Site exposes for accessing available data.
See [The self-hosting installation guide](https://docs.foxglove.dev/docs/primary-sites/self-hosting/installation#configure-foxglove)
for details.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## SiteInboxNotificationTokens
<details><summary><code>client.site_inbox_notification_tokens.<a href="/src/api/resources/site_inbox_notification_tokens/client.rs">list_inbox_notification_tokens</a>(site_id: Option<Option<String>>) -> Result<Vec<InboxNotificationToken>, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .site_inbox_notification_tokens
        .list_inbox_notification_tokens(
            &ListInboxNotificationTokensQueryRequest {
                site_id: Some("siteId".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**site_id:** `Option<String>` — Restrict responses to site with this ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.site_inbox_notification_tokens.<a href="/src/api/resources/site_inbox_notification_tokens/client.rs">create_a_site_inbox_notification_token</a>(request: PostSiteInboxNotificationTokensRequest) -> Result<PostSiteInboxNotificationTokensResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint returns a `token` which can be used to authenticate push notifications
from your inbox bucket to the Foxglove API. This token should be used as a query argument
to the `/endpoints/inbox-notifications` route of this domain, eg.

```
https://api.foxglove.dev/endpoints/inbox-notifications?token=<token>
```

See the [Primary Site
Installation](https://docs.foxglove.dev/docs/primary-sites/self-hosting/installation#bucket-push-notification)
documentation for more details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .site_inbox_notification_tokens
        .create_a_site_inbox_notification_token(
            &PostSiteInboxNotificationTokensRequest {
                site_id: "siteId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**site_id:** `String` — Site associated with the newly created token
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.site_inbox_notification_tokens.<a href="/src/api/resources/site_inbox_notification_tokens/client.rs">delete_an_inbox_notification_token</a>(id: String) -> Result<DeleteSiteInboxNotificationTokensIdResponse, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .site_inbox_notification_tokens
        .delete_an_inbox_notification_token(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Inbox notification token ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## DeviceTokens
<details><summary><code>client.device_tokens.<a href="/src/api/resources/device_tokens/client.rs">list_device_tokens</a>(device_id: Option<Option<String>>) -> Result<Vec<DeviceToken>, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .device_tokens
        .list_device_tokens(
            &ListDeviceTokensQueryRequest {
                device_id: Some("deviceId".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**device_id:** `Option<String>` — Filter by device ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.device_tokens.<a href="/src/api/resources/device_tokens/client.rs">create_a_device_token</a>(request: PostDeviceTokensRequest) -> Result<PostDeviceTokensResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

You must have an Enterprise or Team account to create and use device tokens
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .device_tokens
        .create_a_device_token(
            &PostDeviceTokensRequest {
                device_id: "deviceId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**device_id:** `String` — Device ID associated with the newly created token
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.device_tokens.<a href="/src/api/resources/device_tokens/client.rs">get_a_device_token</a>(id: String) -> Result<DeviceToken, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .device_tokens
        .get_a_device_token(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Device token ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.device_tokens.<a href="/src/api/resources/device_tokens/client.rs">delete_a_device_token</a>(id: String) -> Result<DeleteDeviceTokensIdResponse, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .device_tokens
        .delete_a_device_token(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Device token ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.device_tokens.<a href="/src/api/resources/device_tokens/client.rs">edit_a_device_token</a>(id: String, request: PatchDeviceTokensIdRequest) -> Result<DeviceToken, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .device_tokens
        .edit_a_device_token(
            &"id".to_string(),
            &PatchDeviceTokensIdRequest { enabled: true },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Device token ID
    
</dd>
</dl>

<dl>
<dd>

**enabled:** `bool` — Enables or disables the device token.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## SiteTokens
<details><summary><code>client.site_tokens.<a href="/src/api/resources/site_tokens/client.rs">list_site_tokens</a>(site_id: Option<Option<String>>) -> Result<Vec<SiteToken>, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .site_tokens
        .list_site_tokens(
            &ListSiteTokensQueryRequest {
                site_id: Some("siteId".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**site_id:** `Option<String>` — Filter by site ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.site_tokens.<a href="/src/api/resources/site_tokens/client.rs">create_a_site_token</a>(request: PostSiteTokensRequest) -> Result<PostSiteTokensResponse, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .site_tokens
        .create_a_site_token(
            &PostSiteTokensRequest {
                site_id: "siteId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**site_id:** `String` — Site associated with the newly created token
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.site_tokens.<a href="/src/api/resources/site_tokens/client.rs">delete_a_site_token</a>(id: String) -> Result<GenericSuccess, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use foxglove::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = FoxgloveClient::new(config).expect("Failed to build client");
    client
        .site_tokens
        .delete_a_site_token(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Site token ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>
