use chrono::{DateTime, Utc};
use serde;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PushNotificationPlatform {
    #[serde(rename = "WEB")]
    Web,
    #[serde(rename = "ANDROID")]
    Android,
    #[serde(rename = "IOS")]
    IOS,
    #[serde(rename = "IPADOS")]
    IpadOS,
    #[serde(rename = "WINDOWS")]
    Windows,
    #[serde(rename = "MACOS")]
    MacOS,
    #[serde(rename = "LINUX")]
    Linux,
    #[serde(rename = "CLI")]
    CLI,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PushNotificationToken {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub platform: PushNotificationPlatform,
    pub created_at: DateTime<Utc>,
}
