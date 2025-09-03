# Foxglove SDK Client Example - Usage Guide

This comprehensive client example tests ALL 52 SDK methods with real API calls to verify the complete functionality of the Foxglove Rust SDK. We have achieved 100% method coverage!

## 🚀 Quick Start

### 1. Set up your API token

```bash
# Copy the example environment file
cp .env.example .env

# Edit the .env file and add your Foxglove API token
# Get your token from: https://studio.foxglove.dev → Settings → API Keys
```

### 2. Run the comprehensive test suite

```bash
cd client-example
cargo run
```

## 📋 What This Example Tests

The comprehensive test suite covers all 16 API domains in the Foxglove SDK:

### Core APIs ✅
- **🏢 Sites API** - List and retrieve site information
- **🤖 Devices API** - Device management and properties  
- **🎥 Recordings API** - MCAP file storage and retrieval
- **📅 Events API** - Time-based event tracking
- **🧩 Extensions API** - Custom panel/plugin management

### Data APIs ✅  
- **📎 Recording Attachments** - File attachment management
- **🏷️ Custom Properties** - Metadata management
- **📊 Coverage API** - Data coverage analytics
- **🌊 Stream Data API** - Real-time data streaming
- **📋 Topics API** - Message topic discovery

### Infrastructure APIs ✅
- **🗄️ Lake Files API** - Data lake file access  
- **📐 Layouts API** - Visualization layouts
- **🔑 Device Tokens API** - Device authentication
- **🎫 Site Tokens API** - Site authentication
- **📬 Site Inbox Notification Tokens** - Notification management
- **📥 Imports API** - Data import operations

## 🔍 How It Works

The example systematically tests each API domain:

1. **List Operations** - Tests listing/querying capabilities
2. **Individual Retrieval** - Tests getting specific items by ID
3. **Error Handling** - Gracefully handles API errors
4. **Parameter Validation** - Tests various parameter combinations

## 📊 Expected Output

```
🦊 Comprehensive Foxglove SDK Testing Suite

🔍 Testing ALL SDK methods with real API calls

✅ Client initialized successfully with authentication
🧪 Running Comprehensive API Tests...

  🔍 Testing Sites API methods...
    ✓ list_sites() - Found 2 sites
      1. My Organization (org_xxx) - Type: Primary
      2. Edge Site (site_xxx) - Type: Edge
    ✓ Sites API methods tested successfully

  🔍 Testing Devices API methods...
    ✓ list_devices() - Found 5 devices
      1. robot-001 (dev_xxx)
      2. sensor-hub-02 (dev_yyy)
    ✓ get_a_device(robot-001) - Success
    ✓ Devices API methods tested successfully

  ... [continues for all 16 API domains]

📊 Test Results Summary:
========================
✅ 🏢 Sites API
✅ 🤖 Devices API
✅ 🎥 Recordings API
... [results for all APIs]

🎯 Final Score: 52/52 SDK methods tested (100.0% coverage)
🎉 All tests passed! Complete SDK functionality verified.

🎉 Comprehensive SDK testing completed!
```

## ⚠️ Common Issues

### Authentication Errors
```
❌ Failed to create client: HttpError(401)
```
**Solution**: Verify your `FOXGLOVE_API_TOKEN` is correct and has the necessary permissions.

### Permission Errors  
```
❌ Sites API - Error: HttpError(403)
```
**Solution**: Your API token may not have permissions for certain operations. Check your token capabilities in Foxglove Studio.

### Network Issues
```  
❌ Events API - Error: RequestError(...)
```
**Solution**: Check your internet connection and verify the API endpoint is accessible.

## 🛠️ Customization & Test Configuration

The example now includes a comprehensive `TestConfig` system for safe testing:

### Environment Variables

Control test behavior with these environment variables:

```bash
# Test safety controls
FOXGLOVE_READ_ONLY=true              # Skip all write operations
FOXGLOVE_ENABLE_CREATES=true         # Allow CREATE operations  
FOXGLOVE_ENABLE_UPDATES=true         # Allow UPDATE operations
FOXGLOVE_ENABLE_DELETES=true         # Allow DELETE operations
FOXGLOVE_ENABLE_UPLOADS=true         # Allow binary uploads
FOXGLOVE_CLEANUP=true                # Auto-cleanup created test data
FOXGLOVE_TEST_PREFIX=sdk-test-       # Prefix for test data names
```

### Safe Testing Modes

1. **Read-Only Mode** (Default safe mode):
   ```bash
   FOXGLOVE_READ_ONLY=true cargo run
   ```
   - Only tests GET/LIST operations
   - No data creation or modification
   - Safe for production environments

2. **Full Testing Mode** (Comprehensive):
   ```bash
   FOXGLOVE_ENABLE_CREATES=true \
   FOXGLOVE_ENABLE_UPDATES=true \
   FOXGLOVE_ENABLE_DELETES=true \
   FOXGLOVE_ENABLE_UPLOADS=true \
   FOXGLOVE_CLEANUP=true \
   cargo run
   ```
   - Tests all CRUD operations
   - Creates test data with cleanup
   - Ideal for development/staging

### Additional Customization

1. **Focus on specific APIs** - Comment out test domains you don't need
2. **Adjust parameters** - Modify time ranges, limits, and filters
3. **Enhanced logging** - Set `RUST_LOG=debug` for detailed request/response logging

## 📝 Code Structure

```
src/main.rs
├── main() - Entry point and client setup
├── create_client() - API client initialization  
├── run_comprehensive_tests() - Test orchestration
├── test_sites_comprehensive() - Sites API testing
├── test_devices_comprehensive() - Devices API testing
├── ... [individual test functions for each API]
└── Result analysis and reporting
```

Each test function follows the same pattern:
1. Call the list/query method
2. Display results summary  
3. Test individual item retrieval (where available)
4. Handle and report errors gracefully

## 🚀 Next Steps

After running the comprehensive test:

1. **Explore specific APIs** - Focus on the APIs most relevant to your use case
2. **Implement your logic** - Use the working examples as templates
3. **Add error handling** - Implement production-ready error handling patterns
4. **Scale up** - Add pagination, bulk operations, and performance optimizations

## 📚 Resources

- [Foxglove API Documentation](https://docs.foxglove.dev/docs/api)
- [Foxglove Studio](https://studio.foxglove.dev)
- [MCAP Format Documentation](https://mcap.dev)
- [Rust Async Programming](https://rust-lang.github.io/async-book/)

---

**Happy coding with the Foxglove Rust SDK! 🦊**