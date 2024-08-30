pub mod authentication;
pub mod authorization;
pub mod chat;
pub mod conduit;
pub mod database;
pub mod email;
pub mod forms;
pub mod push_notifications;
pub mod router;
pub mod sms;
pub mod storage;
pub mod unknown;

use std::sync::Arc;

#[derive(Clone)]
pub enum Module {
    Authentication(Arc<authentication::AuthenticationModule>),
    Authorization(Arc<authorization::AuthorizationModule>),
    Chat(Arc<chat::ChatModule>),
    Database(Arc<database::DatabaseModule>),
    Email(Arc<email::EmailModule>),
    Forms(Arc<forms::FormsModule>),
    PushNotifications(Arc<push_notifications::PushNotificationsModule>),
    Router(Arc<router::RouterModule>),
    SMS(Arc<sms::SmsModule>),
    Storage(Arc<storage::StorageModule>),
    Unknown(Arc<unknown::UnknownModule>),
}

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Module::Authentication(m) => (**m).fmt(f),
            Module::Authorization(m) => (**m).fmt(f),
            Module::Chat(m) => (**m).fmt(f),
            Module::Database(m) => (**m).fmt(f),
            Module::Email(m) => (**m).fmt(f),
            Module::Forms(m) => (**m).fmt(f),
            Module::PushNotifications(m) => (**m).fmt(f),
            Module::Router(m) => (**m).fmt(f),
            Module::SMS(m) => (**m).fmt(f),
            Module::Storage(m) => (**m).fmt(f),
            Module::Unknown(m) => (**m).fmt(f),
        }
    }
}
