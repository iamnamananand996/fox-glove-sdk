use chrono::Utc;
use foxglove_api::{api::ApiClient, ClientConfig, DeviceName, GetDevicesRequestSortOrder, ListDevicesQueryRequest, PostDevicesRequest};

#[tokio::main]
async fn main() {
    // Option 1: Use default environment
    let config = ClientConfig {
        token: Some("fox_sk_TlSNDKpFVJb2Emjob2tmfotMp1x96znD".to_string()),
        ..Default::default()
    };

    // Option 2: Use custom URL (uncomment to use)
    // let custom_url = Some("https://api.custom-foxglove.com");
    // let config = ClientConfig {
    //     base_url: custom_url.unwrap_or_else(|| Environment::default().url()).to_string(),
    //     api_key: Some("fox_sk_TlSNDKpFVJb2Emjob2tmfotMp1x96znD".to_string()),
    //     ..Default::default()
    // };

    let client = ApiClient::new(config).unwrap();

    // Example 1: List coverage for last 30 days - using default values
    let end_time = Some(Utc::now());
    let start_time = Some(end_time.unwrap() - chrono::Duration::days(30));

    // let params = RequestParams {
    //     start: start_time,
    //     end: end_time,
    //     ..Default::default()
    // };

    // println!("Fetching coverage data for last 30 days...");
    // let coverage = client
    //     .coverage
    //     .list_coverage(
    //         start_time, end_time, None, None, None, None, None, None, None, None,
    //     )
    //     .await
    //     .unwrap();

    println!(
        "Making request with sort_order: {:?}",
        GetDevicesRequestSortOrder::Asc
    );

    let data = ListDevicesQueryRequest {
        limit : Some(5.0),
        ..Default::default()
    };

    let devices = client
        .devices
        .list_devices(
            &data, // offset
            None, // request_options
        )
        .await
        .unwrap();

    // println!("Found {:?} coverage entries", devices);

    for device in devices {
        println!("{:?}", device)
    }



    // let request = PostDevicesRequest { name: DeviceName("Device1".to_string()), properties: None };

    


    
    // let data = client.devices.delete_a_device(&"dev_0dqbHCph2AE69VUr".to_string(), None).await.unwrap();
    // let data = client.devices.create_a_device(&request, None).await.unwrap();
    // println!("{:?}", data)
}
