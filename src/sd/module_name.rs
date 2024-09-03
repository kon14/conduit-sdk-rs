use serde::{Deserialize, Serialize};

use crate::modules::{
    authentication::AuthenticationModule, authorization::AuthorizationModule, chat::ChatModule,
    database::DatabaseModule, email::EmailModule, forms::FormsModule,
    push_notifications::PushNotificationsModule, router::RouterModule, sms::SmsModule,
    storage::StorageModule,
};

impl AuthenticationModule {
    pub const SERVICE_DISCOVERY_MODULE_NAME: &'static ModuleName = &ModuleName::Authentication;
}

impl AuthorizationModule {
    pub const SERVICE_DISCOVERY_MODULE_NAME: &'static ModuleName = &ModuleName::Authorization;
}

impl ChatModule {
    pub const SERVICE_DISCOVERY_MODULE_NAME: &'static ModuleName = &ModuleName::Chat;
}

impl DatabaseModule {
    pub const SERVICE_DISCOVERY_MODULE_NAME: &'static ModuleName = &ModuleName::Database;
}

impl EmailModule {
    pub const SERVICE_DISCOVERY_MODULE_NAME: &'static ModuleName = &ModuleName::Email;
}

impl FormsModule {
    pub const SERVICE_DISCOVERY_MODULE_NAME: &'static ModuleName = &ModuleName::Forms;
}

impl PushNotificationsModule {
    pub const SERVICE_DISCOVERY_MODULE_NAME: &'static ModuleName = &ModuleName::PushNotifications;
}

impl RouterModule {
    pub const SERVICE_DISCOVERY_MODULE_NAME: &'static ModuleName = &ModuleName::Router;
}

impl SmsModule {
    pub const SERVICE_DISCOVERY_MODULE_NAME: &'static ModuleName = &ModuleName::SMS;
}

impl StorageModule {
    pub const SERVICE_DISCOVERY_MODULE_NAME: &'static ModuleName = &ModuleName::Storage;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModuleName {
    Authentication,
    Authorization,
    Chat,
    Database,
    Email,
    Forms,
    PushNotifications,
    Router,
    SMS,
    Storage,
    Unknown(String),
}

impl From<&str> for ModuleName {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl std::str::FromStr for ModuleName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "authentication" => Ok(ModuleName::Authentication),
            "authorization" => Ok(ModuleName::Authorization),
            "chat" => Ok(ModuleName::Chat),
            "database" => Ok(ModuleName::Database),
            "email" => Ok(ModuleName::Email),
            "forms" => Ok(ModuleName::Forms),
            "pushNotifications" => Ok(ModuleName::PushNotifications),
            "router" => Ok(ModuleName::Router),
            "sms" => Ok(ModuleName::SMS),
            "storage" => Ok(ModuleName::Storage),
            _ => Ok(ModuleName::Unknown(s.to_string())),
        }
    }
}

impl AsRef<str> for ModuleName {
    fn as_ref(&self) -> &str {
        match self {
            ModuleName::Authentication => "authentication",
            ModuleName::Authorization => "authorization",
            ModuleName::Chat => "chat",
            ModuleName::Database => "database",
            ModuleName::Email => "email",
            ModuleName::Forms => "forms",
            ModuleName::PushNotifications => "pushNotifications",
            ModuleName::Router => "router",
            ModuleName::SMS => "sms",
            ModuleName::Storage => "storage",
            ModuleName::Unknown(name) => name,
        }
    }
}

impl std::fmt::Display for ModuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
