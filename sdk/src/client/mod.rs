use crate::{ApiError, ClientConfig};

pub mod coverage;
pub mod custom_properties;
pub mod device_tokens;
pub mod devices;
pub mod events;
pub mod extensions;
pub mod imports;
pub mod lake_files;
pub mod layouts;
pub mod recording_attachments;
pub mod recordings;
pub mod site_inbox_notification_tokens;
pub mod site_tokens;
pub mod sites;
pub mod stream_data;
pub mod topics;
pub struct ApiClient {
    pub config: ClientConfig,
    pub devices: DevicesClient,
    pub recordings: RecordingsClient,
    pub imports: ImportsClient,
    pub recording_attachments: RecordingAttachmentsClient,
    pub coverage: CoverageClient,
    pub stream_data: StreamDataClient,
    pub topics: TopicsClient,
    pub extensions: ExtensionsClient,
    pub events: EventsClient,
    pub lake_files: LakeFilesClient,
    pub layouts: LayoutsClient,
    pub custom_properties: CustomPropertiesClient,
    pub sites: SitesClient,
    pub site_inbox_notification_tokens: SiteInboxNotificationTokensClient,
    pub device_tokens: DeviceTokensClient,
    pub site_tokens: SiteTokensClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            devices: DevicesClient::new(config.clone())?,
            recordings: RecordingsClient::new(config.clone())?,
            imports: ImportsClient::new(config.clone())?,
            recording_attachments: RecordingAttachmentsClient::new(config.clone())?,
            coverage: CoverageClient::new(config.clone())?,
            stream_data: StreamDataClient::new(config.clone())?,
            topics: TopicsClient::new(config.clone())?,
            extensions: ExtensionsClient::new(config.clone())?,
            events: EventsClient::new(config.clone())?,
            lake_files: LakeFilesClient::new(config.clone())?,
            layouts: LayoutsClient::new(config.clone())?,
            custom_properties: CustomPropertiesClient::new(config.clone())?,
            sites: SitesClient::new(config.clone())?,
            site_inbox_notification_tokens: SiteInboxNotificationTokensClient::new(config.clone())?,
            device_tokens: DeviceTokensClient::new(config.clone())?,
            site_tokens: SiteTokensClient::new(config.clone())?,
        })
    }
}

pub use coverage::CoverageClient;
pub use custom_properties::CustomPropertiesClient;
pub use device_tokens::DeviceTokensClient;
pub use devices::DevicesClient;
pub use events::EventsClient;
pub use extensions::ExtensionsClient;
pub use imports::ImportsClient;
pub use lake_files::LakeFilesClient;
pub use layouts::LayoutsClient;
pub use recording_attachments::RecordingAttachmentsClient;
pub use recordings::RecordingsClient;
pub use site_inbox_notification_tokens::SiteInboxNotificationTokensClient;
pub use site_tokens::SiteTokensClient;
pub use sites::SitesClient;
pub use stream_data::StreamDataClient;
pub use topics::TopicsClient;
