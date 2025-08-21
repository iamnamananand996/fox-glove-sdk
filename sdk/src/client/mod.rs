use crate::{ClientConfig, ClientError};

pub mod devices;
pub mod recordings;
pub mod imports;
pub mod recording_attachments;
pub mod coverage;
pub mod stream_data;
pub mod topics;
pub mod extensions;
pub mod events;
pub mod lake_files;
pub mod layouts;
pub mod custom_properties;
pub mod sites;
pub mod site_inbox_notification_tokens;
pub mod device_tokens;
pub mod site_tokens;
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
    pub fn new(config: ClientConfig) -> Result<Self, ClientError> {
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
            site_tokens: SiteTokensClient::new(config.clone())?
        })
    }

}

pub use devices::DevicesClient;
pub use recordings::RecordingsClient;
pub use imports::ImportsClient;
pub use recording_attachments::RecordingAttachmentsClient;
pub use coverage::CoverageClient;
pub use stream_data::StreamDataClient;
pub use topics::TopicsClient;
pub use extensions::ExtensionsClient;
pub use events::EventsClient;
pub use lake_files::LakeFilesClient;
pub use layouts::LayoutsClient;
pub use custom_properties::CustomPropertiesClient;
pub use sites::SitesClient;
pub use site_inbox_notification_tokens::SiteInboxNotificationTokensClient;
pub use device_tokens::DeviceTokensClient;
pub use site_tokens::SiteTokensClient;
