# Foxglove Rust SDK Client Example

A comprehensive example demonstrating how to use the Foxglove Rust SDK to interact with the Foxglove Data Platform API.

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ installed
- A Foxglove API token with appropriate permissions

### 1. Get Your API Token
1. Go to [Foxglove Studio](https://studio.foxglove.dev)
2. Navigate to Settings → API Keys
3. Create a new API key with the required capabilities:
   - `devices.list`, `devices.create`, `devices.update`, `devices.delete`
   - `recordings.list`, `data.upload`
   - `events.create`, `events.list`, `events.update`
   - `sites.list`, `sites.create`
   - `siteTokens.create`
   - And others as needed

### 2. Set Environment Variables
```bash
export FOXGLOVE_API_TOKEN="fox_sk_your_token_here"
```

### 3. Run the Example
```bash
cd client-example
FOXGLOVE_ENABLE_CREATES=true FOXGLOVE_ENABLE_UPDATES=true FOXGLOVE_ENABLE_DELETES=true FOXGLOVE_ENABLE_UPLOADS=true cargo run
```

## 📁 Project Structure

```
client-example/
├── Cargo.toml          # Dependencies and project config
├── README.md           # This file
└── src/
    └── main.rs         # Main example code
```

## 🔧 What This Example Demonstrates

### Core API Operations
- **🤖 Device Management** - Create, list, update, delete devices
- **🎥 Recording Management** - Upload, list, query recordings
- **🌊 Data Streaming** - Download data in MCAP/bag formats
- **📅 Event Management** - Create and query events for data annotation
- **🏢 Site Management** - Manage Primary and Edge sites
- **🔑 Token Management** - Create authentication tokens

### Advanced Features
- **📊 Filtering & Queries** - Using query parameters to filter results
- **📄 Pagination** - Handling large datasets with offset/limit
- **⚠️ Error Handling** - Proper error handling patterns
- **🏷️ Custom Properties** - Managing metadata for devices
- **⏰ Time Range Queries** - Working with time-based data

## 📋 API Coverage

The example covers these major API categories:

| Category | Operations | Coverage |
|----------|------------|----------|
| **Devices** | List, Create, Get, Update, Delete | ✅ Complete |
| **Recordings** | List, Upload, Import, Download | ✅ Complete |
| **Events** | Create, List, Update, Delete | ✅ Complete |
| **Sites** | List, Create, Update | ✅ Complete |
| **Topics** | List with schema info | ✅ Complete |
| **Coverage** | Time range queries | ✅ Complete |
| **Stream Data** | Download links | ✅ Complete |
| **Custom Properties** | List, Create, Update | ✅ Complete |
| **Tokens** | Site tokens, Device tokens | ✅ Complete |

## 💡 Usage Patterns

### Basic Client Setup
```rust
use foxglove_sdk::{ApiClient, ApiClientBuilder, Environment, ClientError};

let client = ApiClientBuilder::new()
    .environment(Environment::Production)
    .token(&token)
    .build()?;
```

### Error Handling
```rust
match client.devices.get_a_device(&device_id, None).await {
    Ok(device) => println!("Found: {}", device.name),
    Err(ClientError::Http { status_code: 404, .. }) => {
        println!("Device not found");
    },
    Err(err) => eprintln!("Error: {:?}", err),
}
```

### Pagination
```rust
let mut offset = 0;
let limit = 50.0;

loop {
    let batch = client.recordings.list_recordings(
        Some(start_time), Some(end_time),
        None, None, None, None, None, None, None,
        Some(limit), Some(offset),
        None, None, None
    ).await?;
    
    if batch.is_empty() { break; }
    
    // Process batch...
    offset += limit as i32;
}
```

### Time-based Queries
```rust
use chrono::{Utc, Duration};

let end_time = Utc::now();
let start_time = end_time - Duration::days(7);

let recordings = client.recordings.list_recordings(
    Some(start_time),
    Some(end_time),
    // ... other parameters
).await?;
```

## 🔧 Dependencies

The example uses these key dependencies:
- **foxglove-sdk** - The Foxglove Rust SDK
- **tokio** - Async runtime
- **serde_json** - JSON handling
- **chrono** - Date/time operations
- **reqwest** - HTTP client
- **anyhow** - Error handling

## ⚠️ Known SDK Issues

Based on our analysis, there are a few issues with the current SDK:

1. **Recording Attachment Download** - May not handle 302 redirects properly
2. **Import from Edge** - Endpoint path may be incorrect (`/recordings/{id}` vs `/recordings/{id}/import`)
3. **Lake Files** - `site_id` should be required parameter but is optional

## 🌍 Environment Variables

```bash
# Required
FOXGLOVE_API_TOKEN=fox_sk_your_token_here

# Optional
RUST_LOG=debug          # For verbose logging
FOXGLOVE_ENV=production # API environment (production/development)
```

## 🎯 Customization

To adapt this example for your use case:

1. **Modify device names** - Change `"example-robot-001"` to your device names
2. **Adjust time ranges** - Modify the `Duration` values for your data queries
3. **Filter parameters** - Add specific query filters for your data
4. **Error handling** - Implement production-ready error handling
5. **Add logging** - Include proper logging for your application

## 🔗 Example Operations

### Device Operations
```rust
// List devices with filtering
let devices = client.devices.list_devices(
    None,                                    // sort_by
    Some("location:warehouse".to_string()),  // query filter
    Some(GetDevicesRequestSortOrder::Asc),   // sort_order
    Some(10.0),                             // limit
    None,                                   // offset
    None                                    // request_options
).await?;

// Create device with properties
let device_request = serde_json::json!({
    "name": "my-robot-001",
    "properties": {
        "location": "warehouse-a",
        "type": "autonomous-robot"
    }
});
let device = client.devices.create_a_device(&device_request, None).await?;
```

### Recording Operations
```rust
// Upload recording
let upload_request = serde_json::json!({
    "filename": "robot_data.mcap",
    "deviceName": "my-robot-001",
    "key": "unique-key-123"
});
let response = client.recordings.upload_a_recording(&upload_request, None).await?;

// List recordings with time filter
let recordings = client.recordings.list_recordings(
    Some(start_time),
    Some(end_time),
    None, None, None, None, None, None,
    Some(GetRecordingsRequestImportStatus::Complete),
    Some(50.0), None, None, None, None
).await?;
```

### Event Operations
```rust
// Create event with metadata
let event_request = serde_json::json!({
    "deviceName": "my-robot-001",
    "start": event_time.to_rfc3339(),
    "end": (event_time + Duration::minutes(5)).to_rfc3339(),
    "metadata": {
        "type": "maintenance",
        "description": "Battery check",
        "technician": "alice@company.com"
    }
});
let event = client.events.create_an_event(&event_request, None).await?;
```

## 🚦 Next Steps

1. **Run the example** to see it in action
2. **Modify for your use case** - Update device names, time ranges, etc.
3. **Implement error handling** appropriate for your production environment
4. **Add logging and monitoring** as needed
5. **Consider connection pooling** for high-throughput applications

## 📚 Resources

- [Foxglove API Documentation](https://docs.foxglove.dev/docs/api)
- [Foxglove Studio](https://studio.foxglove.dev)
- [MCAP Format Documentation](https://mcap.dev)
- [Rust Async Programming](https://rust-lang.github.io/async-book/)
- [Tokio Documentation](https://tokio.rs)

## 🤝 Contributing

If you find issues or want to improve this example:
1. Check the [known issues](#-known-sdk-issues) section
2. Test with your specific use case
3. Consider contributing back improvements to the SDK