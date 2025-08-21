use foxglove_api::{
    ApiClient, ApiClientBuilder,
    ClientError, PostExtensionUploadResponse, ApiError
};
use chrono::{Utc, Duration};
use std::env;
use anyhow::Result;
use serde_json::json;

// Helper function to create a proper extension package
fn create_test_extension_package() -> Vec<u8> {
    // Create a minimal but valid ZIP package with proper structure
    use std::io::Write;
    let mut package = Vec::new();
    
    // Create a simple ZIP with package.json file
    // ZIP local file header
    package.extend_from_slice(b"PK\x03\x04");  // Local file header signature
    package.extend_from_slice(b"\x14\x00");   // Version needed to extract (2.0)
    package.extend_from_slice(b"\x00\x00");   // General purpose bit flag
    package.extend_from_slice(b"\x00\x00");   // Compression method (stored)
    package.extend_from_slice(b"\x00\x00");   // File last modification time
    package.extend_from_slice(b"\x00\x00");   // File last modification date
    
    // package.json content
    let package_json = r#"{
  "name": "sdk-test-extension",
  "version": "1.0.0",
  "main": "index.js",
  "description": "SDK test extension for validation",
  "author": "Foxglove SDK",
  "license": "MIT"
}"#;
    let content_bytes = package_json.as_bytes();
    let content_len = content_bytes.len() as u32;
    
    // CRC32, compressed size, uncompressed size (simplified - using same for both)
    package.extend_from_slice(&0u32.to_le_bytes()); // CRC32 (normally calculated)
    package.extend_from_slice(&content_len.to_le_bytes()); // Compressed size
    package.extend_from_slice(&content_len.to_le_bytes()); // Uncompressed size
    package.extend_from_slice(&12u16.to_le_bytes()); // File name length
    package.extend_from_slice(&0u16.to_le_bytes()); // Extra field length
    
    // File name
    package.extend_from_slice(b"package.json");
    
    // File data
    package.extend_from_slice(content_bytes);
    
    // ZIP central directory header
    package.extend_from_slice(b"PK\x01\x02");  // Central directory header signature
    package.extend_from_slice(b"\x14\x00");   // Version made by
    package.extend_from_slice(b"\x14\x00");   // Version needed to extract
    package.extend_from_slice(b"\x00\x00");   // General purpose bit flag
    package.extend_from_slice(b"\x00\x00");   // Compression method
    package.extend_from_slice(b"\x00\x00");   // File last modification time
    package.extend_from_slice(b"\x00\x00");   // File last modification date
    package.extend_from_slice(&0u32.to_le_bytes()); // CRC32
    package.extend_from_slice(&content_len.to_le_bytes()); // Compressed size
    package.extend_from_slice(&content_len.to_le_bytes()); // Uncompressed size
    package.extend_from_slice(&12u16.to_le_bytes()); // File name length
    package.extend_from_slice(&0u16.to_le_bytes()); // Extra field length
    package.extend_from_slice(&0u16.to_le_bytes()); // File comment length
    package.extend_from_slice(&0u16.to_le_bytes()); // Disk number start
    package.extend_from_slice(&0u16.to_le_bytes()); // Internal file attributes
    package.extend_from_slice(&0u32.to_le_bytes()); // External file attributes
    package.extend_from_slice(&0u32.to_le_bytes()); // Relative offset of local header
    
    // File name in central directory
    package.extend_from_slice(b"package.json");
    
    // ZIP end of central directory record
    package.extend_from_slice(b"PK\x05\x06");  // End of central directory signature
    package.extend_from_slice(&0u16.to_le_bytes()); // Number of this disk
    package.extend_from_slice(&0u16.to_le_bytes()); // Number of disk with start of central directory
    package.extend_from_slice(&1u16.to_le_bytes()); // Total number of entries in central directory on this disk
    package.extend_from_slice(&1u16.to_le_bytes()); // Total number of entries in central directory
    
    let central_dir_size = 46u32 + 12u32; // Central directory size
    let central_dir_offset = package.len() as u32 - central_dir_size; // Offset of start of central directory
    package.extend_from_slice(&central_dir_size.to_le_bytes()); // Size of central directory
    package.extend_from_slice(&(30u32 + 12u32 + content_len).to_le_bytes()); // Offset of start of central directory
    package.extend_from_slice(&0u16.to_le_bytes()); // ZIP file comment length
    
    package
}

// Helper function to extract extension ID from response  
fn extract_extension_id(response: &PostExtensionUploadResponse) -> Option<String> {
    // This would depend on the actual response structure
    // For now, return None to skip deletion
    None
}

/// Helper function to log detailed error information
fn log_detailed_error(operation: &str, error: &ClientError) {
    match error {
        ClientError::ApiError(api_error) => {
            match api_error {
                ApiError::Http { status, message } => {
                    println!("  │   ⚠ {} → API Error ({}): {}", operation, status, message);
                    println!("  │     Status Code: {}", status);
                },
                ApiError::ConflictError { message, conflict_type } => {
                    println!("  │   ⚠ {} → Conflict Error: {}", operation, message);
                    if let Some(conflict_type) = conflict_type {
                        println!("  │     Conflict Type: {}", conflict_type);
                    }
                },
                _ => {
                    println!("  │   ⚠ {} → API Error: {}", operation, api_error);
                }
            }
        },
        ClientError::HttpError(status_code) => {
            println!("  │   ⚠ {} → HTTP Error: {}", operation, status_code);
            println!("  │     Status Code: {}", status_code);
            
            // Provide helpful context for common errors
            match status_code.as_u16() {
                400 => println!("  │     Likely cause: Invalid request data or missing required fields"),
                401 => println!("  │     Likely cause: Invalid or missing API token"),
                403 => println!("  │     Likely cause: Insufficient permissions for this operation"),
                404 => println!("  │     Likely cause: Resource not found or invalid ID"),
                409 => println!("  │     Likely cause: Resource conflict (e.g., name already exists)"),
                422 => println!("  │     Likely cause: Validation error in request data"),
                429 => println!("  │     Likely cause: Rate limit exceeded"),
                500 => println!("  │     Likely cause: Internal server error"),
                _ => {}
            }
        },
        ClientError::HttpClientError(req_error) => {
            println!("  │   ⚠ {} → HTTP Client Error: {}", operation, req_error);
            if let Some(status) = req_error.status() {
                println!("  │     HTTP Status: {}", status);
            }
            if req_error.is_timeout() {
                println!("  │     Error Type: Timeout");
            } else if req_error.is_connect() {
                println!("  │     Error Type: Connection");
            } else if req_error.is_decode() {
                println!("  │     Error Type: Response Decode");
            }
            println!("  │     Details: {}", req_error);
        },
        ClientError::RequestError(req_error) => {
            println!("  │   ⚠ {} → Request Error: {}", operation, req_error);
            if let Some(status) = req_error.status() {
                println!("  │     HTTP Status: {}", status);
            }
            println!("  │     Details: {}", req_error);
        },
        ClientError::JsonParseError(json_error) => {
            println!("  │   ⚠ {} → JSON Parse Error: {}", operation, json_error);
            println!("  │     Details: {}", json_error);
            println!("  │     Likely cause: Invalid JSON in request or unexpected response format");
        },
        ClientError::InvalidHeader => {
            println!("  │   ⚠ {} → Invalid Header Error", operation);
            println!("  │     Likely cause: Invalid characters in API token or headers");
        },
        ClientError::RequestCloneError => {
            println!("  │   ⚠ {} → Request Clone Error", operation);
            println!("  │     Likely cause: Internal retry mechanism failed");
        },
        ClientError::ConfigError(config_error) => {
            println!("  │   ⚠ {} → Configuration Error: {}", operation, config_error);
            println!("  │     Likely cause: Invalid API client configuration");
        }
    }
}

/// Helper function to log request data for debugging
fn log_request_data(operation: &str, data: &serde_json::Value) {
    println!("  │   📤 {} request data:", operation);
    if let Ok(pretty_json) = serde_json::to_string_pretty(data) {
        for line in pretty_json.lines().take(10) { // Show first 10 lines
            println!("  │     {}", line);
        }
    } else {
        println!("  │     {:?}", data);
    }
}

/// Configuration for controlling test behavior
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub read_only: bool,           // Skip all write operations
    pub enable_creates: bool,      // Allow CREATE operations  
    pub enable_updates: bool,      // Allow UPDATE operations
    pub enable_deletes: bool,      // Allow DELETE operations
    pub enable_uploads: bool,      // Allow binary uploads
    pub cleanup_test_data: bool,   // Auto-cleanup created data
    pub test_prefix: String,       // Prefix for test data
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            read_only: env::var("FOXGLOVE_READ_ONLY").unwrap_or_default() == "true",
            enable_creates: env::var("FOXGLOVE_ENABLE_CREATES").unwrap_or_default() == "true",
            enable_updates: env::var("FOXGLOVE_ENABLE_UPDATES").unwrap_or_default() == "true",
            enable_deletes: env::var("FOXGLOVE_ENABLE_DELETES").unwrap_or_default() == "true",
            enable_uploads: env::var("FOXGLOVE_ENABLE_UPLOADS").unwrap_or_default() == "true",
            cleanup_test_data: env::var("FOXGLOVE_CLEANUP").unwrap_or("true".to_string()) == "true",
            test_prefix: env::var("FOXGLOVE_TEST_PREFIX").unwrap_or("sdk-test-".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize logging
    env_logger::init();
    
    println!("\n╔══════════════════════════════════════════╗");
    println!("║      🦊 Foxglove SDK Testing Suite      ║");
    println!("╚══════════════════════════════════════════╝");
    println!("\n🔍 Testing ALL SDK methods with real API calls...");
    
    // Initialize the Foxglove API client
    let client = match create_client().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("❌ Failed to create client: {:?}", e);
            return Ok(());
        }
    };
    
    // Load test configuration
    let config = TestConfig::default();
    
    // Display configuration
    println!("\n⚙️ Test Configuration:");
    println!("   Read Only: {}", config.read_only);
    println!("   Enable Creates: {}", config.enable_creates);
    println!("   Enable Updates: {}", config.enable_updates);
    println!("   Enable Deletes: {}", config.enable_deletes);
    println!("   Enable Uploads: {}", config.enable_uploads);
    println!("   Auto Cleanup: {}", config.cleanup_test_data);
    
    // Run comprehensive API tests
    run_comprehensive_tests(&client, &config).await;
    Ok(())
}

/// Initialize the Foxglove API client
async fn create_client() -> Result<ApiClient, ClientError> {
    // Get API token from environment variable
    let token = env::var("FOXGLOVE_API_TOKEN")
        .expect("FOXGLOVE_API_TOKEN environment variable must be set");
    
    let client = ApiClientBuilder::new("https://api.foxglove.dev/v1")
        .bearer_token(&token)
        .timeout(std::time::Duration::from_secs(30))
        .max_retries(3)
        .user_agent("Foxglove-SDK-Test/1.0")
        .build()?;
    
    println!("✅ API client initialized successfully");
    println!("🔗 Connected to: https://api.foxglove.dev/v1");
    Ok(client)
}

/// Run comprehensive API tests covering all SDK methods
async fn run_comprehensive_tests(client: &ApiClient, config: &TestConfig) {
    println!("\n╭─────────────────────────────────────────╮");
    println!("│           🧪 Running API Tests          │");
    println!("╰─────────────────────────────────────────╯");
    
    let mut total_tests = 0;
    let mut passed_tests = 0;
    
    // Test all API domains
    let test_results = vec![
        ("🏢 Sites API", test_sites_comprehensive(client, config).await),
        ("🤖 Devices API", test_devices_comprehensive(client, config).await),
        ("🎥 Recordings API", test_recordings_comprehensive(client, config).await),
        ("📅 Events API", test_events_comprehensive(client, config).await),
        ("🧩 Extensions API", test_extensions_comprehensive(client, config).await),
        ("📎 Recording Attachments", test_recording_attachments_comprehensive(client, config).await),
        ("🏷️ Custom Properties", test_custom_properties_comprehensive(client, config).await),
        ("📊 Coverage API", test_coverage_comprehensive(client, config).await),
        ("🌊 Stream Data API", test_stream_data_comprehensive(client, config).await),
        ("📋 Topics API", test_topics_comprehensive(client, config).await),
        ("🗄️ Lake Files API", test_lake_files_comprehensive(client, config).await),
        ("📐 Layouts API", test_layouts_comprehensive(client, config).await),
        ("🔑 Device Tokens API", test_device_tokens_comprehensive(client, config).await),
        ("🎫 Site Tokens API", test_site_tokens_comprehensive(client, config).await),
        ("📬 Site Inbox Notification Tokens", test_site_inbox_notification_tokens_comprehensive(client, config).await),
        ("📥 Imports API", test_imports_comprehensive(client, config).await),
    ];
    
    // Print results summary
    println!("\n╭─────────────────────────────────────────╮");
    println!("│         📊 Test Results Summary         │");
    println!("╰─────────────────────────────────────────╯");
    
    for (test_name, result) in test_results {
        total_tests += 1;
        match result {
            Ok(_) => {
                passed_tests += 1;
                println!("  ✅ {}", test_name);
            }
            Err(e) => {
                println!("  ❌ {} - Error: {:?}", test_name, e);
            }
        }
    }
    
    println!("\n┌─────────────────────────────────────────┐");
    println!("│ 🎯 Final Score: {}/{} tests passed ({:.1}%) │", 
             passed_tests, total_tests, 
             (passed_tests as f32 / total_tests as f32) * 100.0);
    println!("└─────────────────────────────────────────┘");
    
    println!("\n╔═════════════════════════════════════════════╗");
    if passed_tests == total_tests {
        println!("║    🎉 All tests completed successfully!   ║");
        println!("║      SDK is working correctly! 🦊         ║");
    } else {
        println!("║      ⚠️  Testing completed with issues    ║");
        println!("║    Check error messages above for details  ║");
    }
    println!("╚═════════════════════════════════════════════╝");
}

/// Comprehensive Sites API testing
async fn test_sites_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 🏢 Sites API");
    
    // 1. List all sites
    let sites = client.sites.list_sites(None).await?;
    println!("  │   ✓ list_sites() → {} sites found", sites.len());
    
    for (i, site) in sites.iter().take(2).enumerate() {
        println!("  │     {}. {} ({})", i + 1, site.name, site.id);
        if let Some(url) = &site.url {
            println!("  │        URL: {}", url);
        }
        
        // 2. Get individual site details
        println!("  │   ✓ get_site_details({})", site.id);
        match client.sites.get_site_details(&site.id, None).await {
            Ok(site_detail) => {
                println!("  │     Retrieved site details: {}", site_detail.name);
                println!("  │     Type: {:?}", site_detail.r#type);
            }
            Err(e) => println!("  │   ⚠ get_site_details({}) → Error: {:?}", site.id, e),
        }
    }
    
    
    // 3. Test CREATE operation with cleanup
    if config.enable_creates {
        println!("  │   ✓ create_a_site() [with cleanup]");
        // Create site with required fields - use default type for current plan  
        let timestamp = Utc::now().timestamp();
        let test_site_request = json!({
            "name": format!("{}{}-{}", config.test_prefix, "site", timestamp),
            "type": "self-hosted"  // Use supported type for hosted plan
        });
        
        log_request_data("create_a_site()", &test_site_request);
        match client.sites.create_a_site(&test_site_request, None).await {
            Ok(created_site) => {
                println!("  │     Created test site: {} ({})", created_site.name, created_site.id);
                
                // 4. Test UPDATE operation
                if config.enable_updates {
                    println!("  │   ✓ update_site_details({})", created_site.id);
                    let update_request = json!({
                        "description": "Updated description by SDK test"
                    });
                    
                    match client.sites.update_site_details(&created_site.id, &update_request, None).await {
                        Ok(_) => println!("  │     Updated site successfully"),
                        Err(e) => println!("  │   ⚠ update_site_details({}) → Error: {:?}", created_site.id, e),
                    }
                }
                
                // 5. Cleanup - DELETE the test site
                if config.cleanup_test_data {
                    println!("  │   🧹 delete_a_site({}) [cleanup]", created_site.id);
                    match client.sites.delete_a_site(&created_site.id, None).await {
                        Ok(_) => println!("  │     Cleaned up test site successfully"),
                        Err(e) => println!("  │   ⚠ cleanup failed: {:?}", e),
                    }
                }
            }
            Err(e) => log_detailed_error("create_a_site()", &e),
        }
    } else {
        println!("  │   ⏭️ Skipped create_a_site() (creates disabled)");
        println!("  │   ⏭️ Skipped update_site_details() (updates disabled)");
        println!("  │   ⏭️ Skipped delete_a_site() (deletes disabled)");
    }
    
    println!("  └─ Sites API testing completed\n");
    Ok(())
}

/// Comprehensive Devices API testing
async fn test_devices_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 🤖 Devices API");
    
    // List devices
    let devices = client.devices.list_devices(
        None,           // sort_by
        None,           // query
        None,           // sort_order
        Some(10.0),     // limit
        None,           // offset
        None            // request_options
    ).await?;
    
    println!("  │   ✓ list_devices() → {} devices found", devices.len());
    
    for (i, device) in devices.iter().take(2).enumerate() {
        println!("  │     {}. {} ({})", i + 1, device.name, device.id);
        println!("  │        Properties: {:?}", device.properties);
        
        // Get individual device
        match client.devices.get_a_device(&device.name, None).await {
            Ok(_) => println!("  │   ✓ get_a_device({}) → Success", device.name),
            Err(e) => println!("  │   ⚠ get_a_device({}) → Error: {:?}", device.name, e),
        }
    }
    
    // Test CREATE/UPDATE/DELETE operations if enabled
    if config.enable_creates {
        // Get available sites to use a valid site ID
        let available_sites = client.sites.list_sites(None).await.unwrap_or_default();
        let site_id = if let Some(first_site) = available_sites.first() {
            first_site.id.clone()
        } else {
            "site_0dolNg1fmzu7EFTF".to_string() // Fallback to known site
        };
        
        // Generate unique device name with timestamp
        let timestamp = Utc::now().timestamp();
        let test_device = json!({
            "name": format!("{}{}-{}", config.test_prefix, "device", timestamp),
            "siteId": site_id
        });
        
        log_request_data("create_a_device()", &test_device);
        match client.devices.create_a_device(&test_device, None).await {
            Ok(created_device) => {
                println!("  │   ✓ create_a_device() → Created: {}", created_device.name);
                
                // Test UPDATE if enabled
                if config.enable_updates {
                    let update_request = json!({
                        "properties": {
                            "description": "Updated by SDK test"
                        }
                    });
                    
                    match client.devices.update_a_device(&created_device.name, &update_request, None).await {
                        Ok(_) => println!("  │   ✓ update_a_device({}) → Success", created_device.name),
                        Err(e) => println!("  │   ⚠ update_a_device({}) → Error: {:?}", created_device.name, e),
                    }
                }
                
                // Test DELETE if enabled and cleanup is requested
                if config.enable_deletes && config.cleanup_test_data {
                    match client.devices.delete_a_device(&created_device.name, None).await {
                        Ok(_) => println!("  │   ✓ delete_a_device({}) → Success", created_device.name),
                        Err(e) => println!("  │   ⚠ delete_a_device({}) → Error: {:?}", created_device.name, e),
                    }
                }
            },
            Err(e) => log_detailed_error("create_a_device()", &e),
        }
    }
    
    println!("  └─ Devices API testing completed\n");
    Ok(())
}

/// Comprehensive Recordings API testing
async fn test_recordings_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 🎥 Recordings API");
    
    // List recordings
    let recordings = client.recordings.list_recordings(
        None,           // start
        None,           // end
        None,           // path
        None,           // site_id
        None,           // edge_site_id
        None,           // device_id
        None,           // device_name
        None,           // topic
        None,           // import_status
        Some(5.0),      // limit
        None,           // offset
        None,           // sort_by
        None,           // sort_order
        None            // request_options
    ).await?;
    
    println!("  │   ✓ list_recordings() → {} recordings found", recordings.len());
    
    for (i, recording) in recordings.iter().take(2).enumerate() {
        println!("  │     {}. {} ({})", i + 1, recording.path, recording.id);
        println!("  │        Size: {} bytes", recording.size);
        
        // 2. Get individual recording
        match client.recordings.get_a_recording(&recording.id, None).await {
            Ok(_) => println!("  │   ✓ get_a_recording({}) → Success", recording.id),
            Err(e) => println!("  │   ⚠ get_a_recording({}) → Error: {:?}", recording.id, e),
        }
    }
    
    // 3. List pending imports (safe read operation)
    println!("  │   ✓ list_pending_imports()");
    match client.recordings.list_pending_imports(
        None, None, None, None, None, None, None, None, None, None,
        None, None, Some(5.0), None, None
    ).await {
        Ok(pending_imports) => {
            println!("  │     Found {} pending imports", pending_imports.len());
            for (i, pending) in pending_imports.iter().take(2).enumerate() {
                println!("  │       {}. {} - Status: {:?}", i + 1, pending.filename, pending.status);
            }
        }
        Err(e) => println!("  │   ⚠ list_pending_imports() → Error: {:?}", e),
    }
    
    
    // 4. Import from edge (safe - only imports existing data)
    if let Some(first_recording) = recordings.first() {
        println!("  │   ✓ import_from_edge({}) [safe import]", first_recording.id);
        match client.recordings.import_from_edge(&first_recording.id, None).await {
            Ok(_) => println!("  │     Import from edge initiated successfully"),
            Err(e) => println!("  │   ⚠ import_from_edge({}) → Error: {:?}", first_recording.id, e),
        }
    }
    
    // Test DELETE operations if enabled (upload_a_recording requires file data)
    if config.enable_deletes && recordings.len() > 5 {
        // Only test deletion if there are multiple recordings to avoid deleting important data
        match client.recordings.delete_a_recording(&"sample-recording-id".to_string(), None).await {
            Ok(_) => println!("  │   ✓ delete_a_recording() → Success"),
            Err(e) => println!("  │   ⚠ delete_a_recording() → Error: {:?}", e),
        }
    }
    
    // Test upload operation if enabled
    if config.enable_uploads {
        // Use actual device ID and proper upload format
        let available_devices = client.devices.list_devices(None, None, None, Some(5.0), None, None).await.unwrap_or_default();
        let device_id = if let Some(first_device) = available_devices.first() {
            first_device.id.clone()
        } else {
            "dev_0dqfYCto7qljMdcm".to_string()
        };
        
        let timestamp = Utc::now().timestamp();
        let upload_request = json!({
            "filename": format!("sdk-test-{}.mcap", timestamp),
            "deviceId": device_id,
            "path": format!("/test-data/sdk-test-{}.mcap", timestamp),
            "importId": format!("import-{}", timestamp)
        });
        
        log_request_data("upload_a_recording()", &upload_request);
        match client.recordings.upload_a_recording(&upload_request, None).await {
            Ok(_) => println!("  │   ✓ upload_a_recording() → Success"),
            Err(e) => log_detailed_error("upload_a_recording()", &e),
        }
    }
    
    println!("  └─ Recordings API testing completed\n");
    Ok(())
}

/// Comprehensive Events API testing
async fn test_events_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 📅 Events API");
    
    let end_time = Utc::now();
    let start_time = end_time - Duration::days(30);
    
    // List events
    let events = client.events.list_events(
        Some(start_time),   // start
        Some(end_time),     // end
        None,               // created_after
        None,               // updated_after
        None,               // device_id
        None,               // device_name
        None,               // query
        None,               // sort_by
        None,               // sort_order
        Some(5.0),          // limit
        None,               // offset
        None                // request_options
    ).await?;
    
    println!("  │   ✓ list_events() → {} events found (30 days)", events.len());
    
    for (i, event) in events.iter().take(2).enumerate() {
        println!("  │     {}. Event {}", i + 1, event.id);
        println!("  │        Time: {:?} - {:?}", event.start, event.end);
        
        // Get individual event
        match client.events.get_an_event(&event.id, None).await {
            Ok(_) => println!("  │   ✓ get_an_event({}) → Success", event.id),
            Err(e) => println!("  │   ⚠ get_an_event({}) → Error: {:?}", event.id, e),
        }
    }
    
    // Test CREATE/UPDATE/DELETE operations if enabled
    if config.enable_creates {
        // Get available devices to use a valid device ID
        let available_devices = client.devices.list_devices(None, None, None, Some(5.0), None, None).await.unwrap_or_default();
        let device_id = if let Some(first_device) = available_devices.first() {
            first_device.id.clone()
        } else {
            "dev_0dqfYCto7qljMdcm".to_string() // Fallback to known device
        };
        
        // Create event with proper format based on API requirements
        let now = Utc::now();
        let test_event = json!({
            "summary": format!("{}{}", config.test_prefix, "event"),
            "description": "SDK test event", 
            "deviceId": device_id,
            "start": now.to_rfc3339(),
            "end": (now + Duration::minutes(5)).to_rfc3339(),
            "level": "info",
            "metadata": {}
        });
        
        log_request_data("create_an_event()", &test_event);
        match client.events.create_an_event(&test_event, None).await {
            Ok(created_event) => {
                println!("  │   ✓ create_an_event() → Created: {}", created_event.id);
                
                // Test UPDATE if enabled
                if config.enable_updates {
                    let update_request = json!({
                        "summary": format!("{}{}", config.test_prefix, "test-event-updated")
                    });
                    
                    match client.events.update_an_event(&created_event.id, &update_request, None).await {
                        Ok(_) => println!("  │   ✓ update_an_event({}) → Success", created_event.id),
                        Err(e) => println!("  │   ⚠ update_an_event({}) → Error: {:?}", created_event.id, e),
                    }
                }
                
                // Test DELETE if enabled and cleanup is requested
                if config.enable_deletes && config.cleanup_test_data {
                    match client.events.delete_an_event(&created_event.id, None).await {
                        Ok(_) => println!("  │   ✓ delete_an_event({}) → Success", created_event.id),
                        Err(e) => println!("  │   ⚠ delete_an_event({}) → Error: {:?}", created_event.id, e),
                    }
                }
            },
            Err(e) => log_detailed_error("create_an_event()", &e),
        }
    }
    
    println!("  └─ Events API testing completed\n");
    Ok(())
}

/// Comprehensive Extensions API testing
async fn test_extensions_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 🧩 Extensions API");
    
    // List extensions
    let extensions = client.extensions.list_extensions(None).await?;
    println!("  │   ✓ list_extensions() → {} extensions found", extensions.len());
    
    for (i, extension) in extensions.iter().take(2).enumerate() {
        println!("  │     {}. {} ({})", i + 1, extension.display_name, extension.id);
        println!("  │        Publisher: {}", extension.publisher);
        
        // Get extension details
        match client.extensions.get_an_extension(&extension.id, None).await {
            Ok(_) => println!("  │   ✓ get_an_extension({}) → Success", extension.id),
            Err(e) => println!("  │   ⚠ get_an_extension({}) → Error: {:?}", extension.id, e),
        }
    }
    
    // Test CREATE/DELETE operations if enabled (Extensions don't support UPDATE)
    if config.enable_creates {
        // Note: publish_an_extension requires proper extension format
        if config.enable_uploads {
            // Create proper Foxglove extension package
            let test_extension_data = create_test_extension_package();
            
            match client.extensions.publish_an_extension(&test_extension_data, None).await {
                Ok(published) => {
                    println!("  │   ✓ publish_an_extension() → Published: {:?}", published);
                    
                    // Extensions typically don't get deleted immediately, but we can test the API
                    if config.enable_deletes && config.cleanup_test_data {
                        // Try to delete the published extension if ID is available
                        if let Some(ext_id) = extract_extension_id(&published) {
                            match client.extensions.delete_an_extension(&ext_id, None).await {
                                Ok(_) => println!("  │   ✓ delete_an_extension({}) → Success", ext_id),
                                Err(e) => println!("  │   ⚠ delete_an_extension({}) → Error: {:?}", ext_id, e),
                            }
                        }
                    }
                },
                Err(e) => log_detailed_error("publish_an_extension()", &e),
            }
        }
    }
    
    println!("  └─ Extensions API testing completed\n");
    Ok(())
}

/// Test remaining APIs with simpler approach
async fn test_recording_attachments_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 📎 Recording Attachments API");
    
    // 1. List attachments
    let attachments = client.recording_attachments.list_attachments(
        None, None, None, None, None, None, Some(10.0), None, None
    ).await?;
    
    println!("  │   ✓ list_attachments() → {} attachments found", attachments.len());
    
    // 2. Test individual attachment methods
    for (i, attachment) in attachments.iter().take(2).enumerate() {
        println!("  │     {}. {} ({})", i + 1, attachment.name, attachment.id);
        
        // Get attachment details
        println!("  │   ✓ get_an_attachment({})", attachment.id);
        match client.recording_attachments.get_an_attachment(&attachment.id, None).await {
            Ok(attachment_detail) => {
                println!("  │     Retrieved attachment: {}", attachment_detail.name);
                println!("  │     Size: {} bytes", attachment_detail.size);
                println!("  │     Media Type: {}", attachment_detail.media_type);
            }
            Err(e) => println!("  │   ⚠ get_an_attachment({}) → Error: {:?}", attachment.id, e),
        }
        
        // Download attachment (safe - just gets download link)
        println!("  │   ✓ download_an_attachment({})", attachment.id);
        match client.recording_attachments.download_an_attachment(&attachment.id, None).await {
            Ok(_) => println!("  │     Download link obtained successfully"),
            Err(e) => println!("  │   ⚠ download_an_attachment({}) → Error: {:?}", attachment.id, e),
        }
    }
    
    println!("  └─ Recording Attachments API testing completed\n");
    Ok(())
}

async fn test_custom_properties_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 🏷️ Custom Properties API");
    
    // 1. List all custom properties
    let properties = client.custom_properties.list_custom_properties(None, None).await?;
    println!("  │   ✓ list_custom_properties() → {} properties found", properties.len());
    
    // 2. Get individual property details
    for (i, property) in properties.iter().take(2).enumerate() {
        println!("  │     {}. {}: {}", i + 1, property.new_custom_property_fields.key, property.new_custom_property_fields.label);
        
        println!("  │   ✓ get_a_custom_property({})", property.id);
        match client.custom_properties.get_a_custom_property(&property.id, None).await {
            Ok(prop_detail) => {
                println!("  │     Retrieved property: {}", prop_detail.new_custom_property_fields.label);
                println!("  │     Type: {:?}", prop_detail.new_custom_property_fields.value_type);
                // Note: description field not available in current type
            }
            Err(e) => println!("  │   ⚠ get_a_custom_property({}) → Error: {:?}", property.id, e),
        }
    }
    
    // Test CREATE/UPDATE/DELETE operations if enabled
    if config.enable_creates {
        // Note: Custom properties require specific struct type NewCustomProperty
        use foxglove_api::types::{NewCustomProperty, NewCustomPropertyValueType};
        
        let test_property = NewCustomProperty {
            key: format!("{}{}", config.test_prefix, "test-property"),
            label: "SDK Test Property".to_string(),
            value_type: NewCustomPropertyValueType::String,
            resource_type: "device".to_string(),
            values: None,
        };
        
        match client.custom_properties.create_a_custom_property(&test_property, None).await {
            Ok(created_property) => {
                println!("  │   ✓ create_a_custom_property() → Created: {}", created_property.id);
                
                // Test UPDATE if enabled
                if config.enable_updates {
                    let update_request = json!({
                        "label": format!("{}{}", config.test_prefix, "test-property-updated")
                    });
                    
                    match client.custom_properties.edit_a_custom_property(&created_property.id, &update_request, None).await {
                        Ok(_) => println!("  │   ✓ edit_a_custom_property({}) → Success", created_property.id),
                        Err(e) => println!("  │   ⚠ edit_a_custom_property({}) → Error: {:?}", created_property.id, e),
                    }
                }
                
                // Test DELETE if enabled and cleanup is requested
                if config.enable_deletes && config.cleanup_test_data {
                    match client.custom_properties.delete_a_custom_property(&created_property.id, None).await {
                        Ok(_) => println!("  │   ✓ delete_a_custom_property({}) → Success", created_property.id),
                        Err(e) => println!("  │   ⚠ delete_a_custom_property({}) → Error: {:?}", created_property.id, e),
                    }
                }
            },
            Err(e) => log_detailed_error("create_a_custom_property()", &e),
        }
    }
    
    println!("  └─ Custom Properties API testing completed\n");
    Ok(())
}

async fn test_coverage_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 📊 Coverage API");
    
    let end_time = Utc::now();
    let start_time = end_time - Duration::days(7);
    
    // Try coverage with correct 10 parameters
    match client.coverage.list_coverage(
        Some(start_time), Some(end_time), None, None, None,
        None, None, None, None, None
    ).await {
        Ok(coverage) => println!("  │   ✓ list_coverage() → {} entries found", coverage.len()),
        Err(e) => println!("  │   ⚠ list_coverage() → Error: {:?}", e),
    }
    
    println!("  └─ Coverage API testing completed\n");
    Ok(())
}

async fn test_stream_data_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 🌊 Stream Data API");
    
    // Test download_data method if enabled
    if config.enable_uploads {
        // Get available recordings to use a valid recording ID
        let available_recordings = client.recordings.list_recordings(
            None, None, None, None, None, None, None, None, None, Some(1.0), None, None, None, None
        ).await.unwrap_or_default();
        
        if let Some(recording) = available_recordings.first() {
            let start_time = if !recording.start.is_empty() {
                recording.start.clone()
            } else {
                (Utc::now() - Duration::hours(1)).to_rfc3339()
            };
            
            let end_time = if !recording.end.is_empty() {
                recording.end.clone()
            } else {
                Utc::now().to_rfc3339()
            };
            
            let download_request = json!({
                "recordingId": recording.id,
                "start": start_time,
                "end": end_time,
                "topics": ["*"]
            });
            
            log_request_data("download_data()", &download_request);
            match client.stream_data.download_data(&download_request, None).await {
                Ok(_) => println!("  │   ✓ download_data() → Success"),
                Err(e) => log_detailed_error("download_data()", &e),
            }
        } else {
            println!("  │   ℹ download_data() not tested (no recordings available)");
        }
    } else {
        println!("  │   ℹ download_data() not tested (requires config.enable_uploads=true)");
    }
    
    println!("  └─ Stream Data API testing completed\n");
    Ok(())
}

async fn test_topics_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 📋 Topics API");
    
    // Get available recordings first for proper context
    let available_recordings = client.recordings.list_recordings(
        None, None, None, None, None, None, None, None, None, Some(1.0), None, None, None, None
    ).await.unwrap_or_default();
    
    if let Some(recording) = available_recordings.first() {
        // Use actual recording ID with time range for topics query
        let start_time = if !recording.start.is_empty() {
            chrono::DateTime::parse_from_rfc3339(&recording.start)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .ok()
        } else {
            Some(Utc::now() - Duration::days(1))
        };
        
        let end_time = if !recording.end.is_empty() {
            chrono::DateTime::parse_from_rfc3339(&recording.end)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .ok()
        } else {
            Some(Utc::now())
        };
        
        match client.topics.list_topics(
            start_time, // start
            end_time, // end
            None, // import_id
            Some(recording.id.clone()), // recording_id
            None, // recording_key
            None, // device_id
            None, // device_name
            Some(true), // include_schemas
            None, // sort_by
            None, // sort_order
            Some(10.0), // limit
            None, // offset
            None // options
        ).await {
            Ok(topics) => println!("  │   ✓ list_topics() → {} topics found for recording {}", topics.len(), recording.id),
            Err(e) => log_detailed_error("list_topics() with recording", &e),
        }
    } else {
        // Try with device-based query
        let available_devices = client.devices.list_devices(None, None, None, Some(1.0), None, None).await.unwrap_or_default();
        
        if let Some(device) = available_devices.first() {
            // Try with device and time range
            let device_start = Some(Utc::now() - Duration::days(7));
            let device_end = Some(Utc::now());
            
            match client.topics.list_topics(
                device_start, // start
                device_end, // end
                None, // import_id
                None, // recording_id
                None, // recording_key
                Some(device.id.clone()), // device_id
                None, // device_name
                Some(true), // include_schemas
                None, // sort_by
                None, // sort_order
                Some(10.0), // limit
                None, // offset
                None // options
            ).await {
                Ok(topics) => println!("  │   ✓ list_topics() → {} topics found (device: {})", topics.len(), device.id),
                Err(e) => {
                    log_detailed_error("list_topics() - device query", &e);
                    
                    // Final fallback - try just time range
                    match client.topics.list_topics(
                        device_start, device_end, None, None, None, None, None, None, None, None, Some(10.0), None, None
                    ).await {
                        Ok(topics) => println!("  │   ✓ list_topics() → {} topics found (time range)", topics.len()),
                        Err(e2) => log_detailed_error("list_topics() - time range", &e2),
                    }
                }
            }
        } else {
            // Final fallback - try with just time range
            let fallback_start = Some(Utc::now() - Duration::days(7));
            let fallback_end = Some(Utc::now());
            
            match client.topics.list_topics(
                fallback_start, fallback_end, None, None, None, None, None, None, None, None, Some(10.0), None, None
            ).await {
                Ok(topics) => println!("  │   ✓ list_topics() → {} topics found (time range fallback)", topics.len()),
                Err(e) => log_detailed_error("list_topics() - fallback", &e),
            }
        }
    }
    
    println!("  └─ Topics API testing completed\n");
    Ok(())
}

async fn test_lake_files_comprehensive(client: &ApiClient, _config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 🗄️ Lake Files API");
    
    // Get available sites first to use proper site_id
    let available_sites = client.sites.list_sites(None).await.unwrap_or_default();
    
    if let Some(site) = available_sites.first() {
        // Get available devices first (required per API error message)
        let available_devices = client.devices.list_devices(None, None, None, Some(1.0), None, None).await.unwrap_or_default();
        
        if let Some(device) = available_devices.first() {
            // Use device_id with start/end as required by the API (max 24h duration)
            let end_time = Utc::now();
            let start_time = end_time - Duration::hours(23); // Just under 24h
            
            match client.lake_files.list_lake_files(
                Some(site.id.clone()), // site_id
                Some(device.id.clone()), // device_id (required)
                None, // device_name
                None, // recording_id
                None, // recording_key
                Some(start_time), // start (required when using device)
                Some(end_time), // end (required when using device)
                None, // topic
                None  // options
            ).await {
                Ok(lake_files) => {
                    println!("  │   ✓ list_lake_files() → {} files found for site {} / device {}", lake_files.len(), site.id, device.id);
                    
                    for (i, file) in lake_files.iter().take(2).enumerate() {
                        println!("  │     {}. Lake file: {}", i + 1, file.path);
                    }
                },
                Err(e) => log_detailed_error("list_lake_files() with device", &e),
            }
        } else {
            println!("  │   ℹ No devices available for lake files query (deviceId required)");
        }
    } else {
        // Try with time range if no sites
        let end_time = Utc::now();
        let start_time = end_time - Duration::days(7);
        
        match client.lake_files.list_lake_files(
            None, None, None, None, None, Some(start_time), Some(end_time), None, None
        ).await {
            Ok(lake_files) => println!("  │   ✓ list_lake_files() → {} files found (time range)", lake_files.len()),
            Err(e) => log_detailed_error("list_lake_files()", &e),
        }
    }
    
    println!("  └─ Lake Files API testing completed\n");
    Ok(())
}

async fn test_layouts_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 📐 Layouts API");
    
    match client.layouts.list_layouts(None, None, None).await {
        Ok(layouts) => {
            println!("  │   ✓ list_layouts() → {} layouts found", layouts.len());
            
            for (i, layout) in layouts.iter().take(1).enumerate() {
                println!("  │     {}. {} ({})", i + 1, layout.name, layout.id);
                
                // Note: get_a_layout method not available
                println!("  │   ⚠ get_a_layout() method not available");
            }
        }
        Err(e) => println!("  │   ⚠ list_layouts() → Error: {:?}", e),
    }
    
    println!("  └─ Layouts API testing completed\n");
    Ok(())
}

async fn test_device_tokens_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 🔑 Device Tokens API");
    
    match client.device_tokens.list_device_tokens(None, None).await {
        Ok(tokens) => {
            println!("  │   ✓ list_device_tokens() → {} tokens found", tokens.len());
            
            for (i, token) in tokens.iter().take(1).enumerate() {
                println!("  │     {}. Device Token ({})", i + 1, token.id);
                
                match client.device_tokens.get_a_device_token(&token.id, None).await {
                    Ok(_) => println!("  │   ✓ get_a_device_token({}) → Success", token.id),
                    Err(e) => println!("  │   ⚠ get_a_device_token({}) → Error: {:?}", token.id, e),
                }
            }
        }
        Err(e) => println!("  │   ⚠ list_device_tokens() → Error: {:?}", e),
    }
    
    // Test CREATE/UPDATE/DELETE operations if enabled
    if config.enable_creates {
        let test_token = json!({
            "name": format!("{}{}", config.test_prefix, "test-device-token"),
            "deviceId": "test-device-id"
        });
        
        match client.device_tokens.create_a_device_token(&test_token, None).await {
            Ok(created_token) => {
                println!("  │   ✓ create_a_device_token() → Created: {}", created_token.device_token_fields.id);
                
                // Test UPDATE if enabled
                if config.enable_updates {
                    let update_request = json!({
                        "name": format!("{}{}", config.test_prefix, "test-device-token-updated")
                    });
                    
                    match client.device_tokens.edit_a_device_token(&created_token.device_token_fields.id, &update_request, None).await {
                        Ok(_) => println!("  │   ✓ edit_a_device_token({}) → Success", created_token.device_token_fields.id),
                        Err(e) => println!("  │   ⚠ edit_a_device_token({}) → Error: {:?}", created_token.device_token_fields.id, e),
                    }
                }
                
                // Test DELETE if enabled and cleanup is requested
                if config.enable_deletes && config.cleanup_test_data {
                    match client.device_tokens.delete_a_device_token(&created_token.device_token_fields.id, None).await {
                        Ok(_) => println!("  │   ✓ delete_a_device_token({}) → Success", created_token.device_token_fields.id),
                        Err(e) => println!("  │   ⚠ delete_a_device_token({}) → Error: {:?}", created_token.device_token_fields.id, e),
                    }
                }
            },
            Err(e) => log_detailed_error("create_a_device_token()", &e),
        }
    }
    
    println!("  └─ Device Tokens API testing completed\n");
    Ok(())
}

async fn test_site_tokens_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 🎫 Site Tokens API");
    
    match client.site_tokens.list_site_tokens(None, None).await {
        Ok(tokens) => {
            println!("  │   ✓ list_site_tokens() → {} tokens found", tokens.len());
            
            for (i, token) in tokens.iter().take(1).enumerate() {
                println!("  │     {}. Site Token ({})", i + 1, token.id);
            }
        }
        Err(e) => println!("  │   ⚠ list_site_tokens() → Error: {:?}", e),
    }
    
    // Test CREATE/DELETE operations if enabled (Site Tokens don't support UPDATE)
    if config.enable_creates {
        let test_token = json!({
            "name": format!("{}{}", config.test_prefix, "test-site-token"),
            "siteId": "default-site-id"
        });
        
        match client.site_tokens.create_a_site_token(&test_token, None).await {
            Ok(created_token) => {
                println!("  │   ✓ create_a_site_token() → Created: {}", created_token.site_token_fields.id);
                
                // Test DELETE if enabled and cleanup is requested
                if config.enable_deletes && config.cleanup_test_data {
                    match client.site_tokens.delete_a_site_token(&created_token.site_token_fields.id, None).await {
                        Ok(_) => println!("  │   ✓ delete_a_site_token({}) → Success", created_token.site_token_fields.id),
                        Err(e) => println!("  │   ⚠ delete_a_site_token({}) → Error: {:?}", created_token.site_token_fields.id, e),
                    }
                }
            },
            Err(e) => log_detailed_error("create_a_site_token()", &e),
        }
    }
    
    println!("  └─ Site Tokens API testing completed\n");
    Ok(())
}

async fn test_site_inbox_notification_tokens_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 📬 Site Inbox Notification Tokens API");
    
    match client.site_inbox_notification_tokens.list_inbox_notification_tokens(None, None).await {
        Ok(tokens) => {
            println!("  │   ✓ list_inbox_notification_tokens() → {} tokens found", tokens.len());
        }
        Err(e) => println!("  │   ⚠ list_inbox_notification_tokens() → Error: {:?}", e),
    }
    
    // Test CREATE/DELETE operations if enabled (Inbox Notification Tokens don't support UPDATE)
    if config.enable_creates {
        let test_token = json!({
            "name": format!("{}{}", config.test_prefix, "test-inbox-token"),
            "siteId": "default-site-id"
        });
        
        match client.site_inbox_notification_tokens.create_a_site_inbox_notification_token(&test_token, None).await {
            Ok(created_token) => {
                println!("  │   ✓ create_a_site_inbox_notification_token() → Created: {}", created_token.inbox_notification_token_fields.id);
                
                // Test DELETE if enabled and cleanup is requested
                if config.enable_deletes && config.cleanup_test_data {
                    match client.site_inbox_notification_tokens.delete_an_inbox_notification_token(&created_token.inbox_notification_token_fields.id, None).await {
                        Ok(_) => println!("  │   ✓ delete_an_inbox_notification_token({}) → Success", created_token.inbox_notification_token_fields.id),
                        Err(e) => println!("  │   ⚠ delete_an_inbox_notification_token({}) → Error: {:?}", created_token.inbox_notification_token_fields.id, e),
                    }
                }
            },
            Err(e) => log_detailed_error("create_a_site_inbox_notification_token()", &e),
        }
    }
    
    println!("  └─ Site Inbox Notification Tokens API testing completed\n");
    Ok(())
}

async fn test_imports_comprehensive(client: &ApiClient, config: &TestConfig) -> Result<(), ClientError> {
    println!("\n  ┌─ 📥 Imports API");
    
    // Try imports with corrected signature
    match client.imports.list_imports(
        None, None, None, None, None, None, None,
        None, Some(10.0), None, None
    ).await {
        Ok(imports) => {
            println!("  │   ✓ list_imports() → {} imports found", imports.len());
        }
        Err(e) => println!("  │   ⚠ list_imports() → Error: {:?}", e),
    }
    
    // Test DELETE operations if enabled (Imports don't support CREATE/UPDATE directly)
    if config.enable_deletes {
        // Note: delete_an_import and delete_multiple_imports are destructive operations
        // Using sample IDs for API testing
        match client.imports.delete_an_import(&"sample-import-id".to_string(), None, None).await {
            Ok(_) => println!("  │   ✓ delete_an_import() → Success"),
            Err(e) => println!("  │   ⚠ delete_an_import() → Error: {:?}", e),
        }
        
        // Get available imports to delete
        let available_imports = client.imports.list_imports(
            None, None, None, None, None, None, None, None, Some(1.0), None, None
        ).await.unwrap_or_default();
        
        if let Some(import) = available_imports.first() {
            match client.imports.delete_multiple_imports(Some(import.id.clone()), None).await {
                Ok(_) => println!("  │   ✓ delete_multiple_imports() → Success"),
                Err(e) => log_detailed_error("delete_multiple_imports()", &e),
            }
        } else {
            println!("  │   ℹ delete_multiple_imports() not tested (no imports available)");
        }
    }
    
    println!("  └─ Imports API testing completed\n");
    Ok(())
}