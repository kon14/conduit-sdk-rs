pub(crate) mod proto {
    include!("../../../proto_gen/pushnotifications.rs");
}

use crate::modules::push_notifications::PushNotificationPlatform;
use std::collections::HashMap;
use tonic::transport::Channel;

pub struct PushNotificationsClient {
    pub(crate) _client: proto::push_notifications_client::PushNotificationsClient<Channel>,
}

impl PushNotificationsClient {
    pub fn new(channel: Channel) -> Self {
        PushNotificationsClient {
            _client: proto::push_notifications_client::PushNotificationsClient::new(channel),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PushNotificationsData {
    pub title: Option<String>,
    pub body: Option<String>,
    pub data: Option<HashMap<String, String>>,
    pub platform: Option<PushNotificationPlatform>,
}

mod get_notification_tokens;
mod send_many_notifications;
mod send_notification;
mod send_notification_to_many_devices;
mod set_notification_token;
