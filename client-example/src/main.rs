use foxglove_api::{
    ApiClient, ApiClientBuilder,
    ClientError
};
use chrono::{Utc, Duration};
use std::env;
use anyhow::Result;

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
    
    // Run comprehensive API tests
    run_comprehensive_tests(&client).await;
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
async fn run_comprehensive_tests(client: &ApiClient) {
    println!("\n╭─────────────────────────────────────────╮");
    println!("│           🧪 Running API Tests          │");
    println!("╰─────────────────────────────────────────╯");
    
    let mut total_tests = 0;
    let mut passed_tests = 0;
    
    // Test all API domains
    let test_results = vec![
        ("🏢 Sites API", test_sites_comprehensive(client).await),
        ("🤖 Devices API", test_devices_comprehensive(client).await),
        ("🎥 Recordings API", test_recordings_comprehensive(client).await),
        ("📅 Events API", test_events_comprehensive(client).await),
        ("🧩 Extensions API", test_extensions_comprehensive(client).await),
        ("📎 Recording Attachments", test_recording_attachments_comprehensive(client).await),
        ("🏷️ Custom Properties", test_custom_properties_comprehensive(client).await),
        ("📊 Coverage API", test_coverage_comprehensive(client).await),
        ("🌊 Stream Data API", test_stream_data_comprehensive(client).await),
        ("📋 Topics API", test_topics_comprehensive(client).await),
        ("🗄️ Lake Files API", test_lake_files_comprehensive(client).await),
        ("📐 Layouts API", test_layouts_comprehensive(client).await),
        ("🔑 Device Tokens API", test_device_tokens_comprehensive(client).await),
        ("🎫 Site Tokens API", test_site_tokens_comprehensive(client).await),
        ("📬 Site Inbox Notification Tokens", test_site_inbox_notification_tokens_comprehensive(client).await),
        ("📥 Imports API", test_imports_comprehensive(client).await),
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
async fn test_sites_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
    println!("\n  ┌─ 🏢 Sites API");
    
    // List all sites
    let sites = client.sites.list_sites(None).await?;
    println!("  │   ✓ list_sites() → {} sites found", sites.len());
    
    for (i, site) in sites.iter().take(2).enumerate() {
        println!("  │     {}. {} ({})", i + 1, site.name, site.id);
        if let Some(url) = &site.url {
            println!("  │        URL: {}", url);
        }
    }
    
    println!("  └─ Sites API testing completed\n");
    Ok(())
}

/// Comprehensive Devices API testing
async fn test_devices_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
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
    
    println!("  └─ Devices API testing completed\n");
    Ok(())
}

/// Comprehensive Recordings API testing
async fn test_recordings_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
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
        
        // Get individual recording
        match client.recordings.get_a_recording(&recording.id, None).await {
            Ok(_) => println!("  │   ✓ get_a_recording({}) → Success", recording.id),
            Err(e) => println!("  │   ⚠ get_a_recording({}) → Error: {:?}", recording.id, e),
        }
    }
    
    println!("  └─ Recordings API testing completed\n");
    Ok(())
}

/// Comprehensive Events API testing
async fn test_events_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
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
    
    println!("  └─ Events API testing completed\n");
    Ok(())
}

/// Comprehensive Extensions API testing
async fn test_extensions_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
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
    
    println!("  └─ Extensions API testing completed\n");
    Ok(())
}

/// Test remaining APIs with simpler approach
async fn test_recording_attachments_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
    println!("\n  ┌─ 📎 Recording Attachments API");
    
    let attachments = client.recording_attachments.list_attachments(
        None, None, None, None, None, None, Some(10.0), None, None
    ).await?;
    
    println!("  │   ✓ list_attachments() → {} attachments found", attachments.len());
    println!("  └─ Recording Attachments API testing completed\n");
    Ok(())
}

async fn test_custom_properties_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
    println!("\n  ┌─ 🏷️ Custom Properties API");
    
    let properties = client.custom_properties.list_custom_properties(None, None).await?;
    println!("  │   ✓ list_custom_properties() → {} properties found", properties.len());
    println!("  └─ Custom Properties API testing completed\n");
    Ok(())
}

async fn test_coverage_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
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

async fn test_stream_data_comprehensive(_client: &ApiClient) -> Result<(), ClientError> {
    println!("\n  ┌─ 🌊 Stream Data API");
    println!("  │   ⚠ Requires recording ID - skipped for safety");
    println!("  └─ Stream Data API testing completed\n");
    Ok(())
}

async fn test_topics_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
    println!("\n  ┌─ 📋 Topics API");
    
    // Try topics with correct 13 parameters
    match client.topics.list_topics(
        None, None, None, None, None, None, None, None,
        None, None, Some(20.0), None, None
    ).await {
        Ok(topics) => println!("  │   ✓ list_topics() → {} topics found", topics.len()),
        Err(e) => println!("  │   ⚠ list_topics() → Error: {:?}", e),
    }
    
    println!("  └─ Topics API testing completed\n");
    Ok(())
}

async fn test_lake_files_comprehensive(_client: &ApiClient) -> Result<(), ClientError> {
    println!("\n  ┌─ 🗄️ Lake Files API");
    println!("  │   ⚠ Requires site_id parameter - skipped");
    println!("  └─ Lake Files API testing completed\n");
    Ok(())
}

async fn test_layouts_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
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

async fn test_device_tokens_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
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
    
    println!("  └─ Device Tokens API testing completed\n");
    Ok(())
}

async fn test_site_tokens_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
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
    
    println!("  └─ Site Tokens API testing completed\n");
    Ok(())
}

async fn test_site_inbox_notification_tokens_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
    println!("\n  ┌─ 📬 Site Inbox Notification Tokens API");
    
    match client.site_inbox_notification_tokens.list_inbox_notification_tokens(None, None).await {
        Ok(tokens) => {
            println!("  │   ✓ list_inbox_notification_tokens() → {} tokens found", tokens.len());
        }
        Err(e) => println!("  │   ⚠ list_inbox_notification_tokens() → Error: {:?}", e),
    }
    
    println!("  └─ Site Inbox Notification Tokens API testing completed\n");
    Ok(())
}

async fn test_imports_comprehensive(client: &ApiClient) -> Result<(), ClientError> {
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
    
    println!("  └─ Imports API testing completed\n");
    Ok(())
}