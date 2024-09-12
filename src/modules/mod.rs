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

use crate::addr::GrpcAddress;
use crate::sd::ModuleName;
use std::sync::Arc;
use tonic::transport::Channel;

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

impl Module {
    pub(crate) fn new(
        module_name: ModuleName,
        grpc_address: GrpcAddress,
        channel: Channel,
    ) -> Self {
        match module_name {
            ModuleName::Authentication => Module::Authentication(Arc::new(
                authentication::AuthenticationModule::new(grpc_address, channel),
            )),
            ModuleName::Authorization => Module::Authorization(Arc::new(
                authorization::AuthorizationModule::new(grpc_address, channel),
            )),
            ModuleName::Chat => {
                Module::Chat(Arc::new(chat::ChatModule::new(grpc_address, channel)))
            }
            ModuleName::Database => Module::Database(Arc::new(database::DatabaseModule::new(
                grpc_address,
                channel,
            ))),
            ModuleName::Email => {
                Module::Email(Arc::new(email::EmailModule::new(grpc_address, channel)))
            }
            ModuleName::Forms => {
                Module::Forms(Arc::new(forms::FormsModule::new(grpc_address, channel)))
            }
            ModuleName::PushNotifications => Module::PushNotifications(Arc::new(
                push_notifications::PushNotificationsModule::new(grpc_address, channel),
            )),
            ModuleName::Router => {
                Module::Router(Arc::new(router::RouterModule::new(grpc_address, channel)))
            }
            ModuleName::SMS => Module::SMS(Arc::new(sms::SmsModule::new(grpc_address, channel))),
            ModuleName::Storage => {
                Module::Storage(Arc::new(storage::StorageModule::new(grpc_address, channel)))
            }
            ModuleName::Unknown(module_name) => Module::Unknown(Arc::new(
                unknown::UnknownModule::new(module_name, grpc_address, channel),
            )),
        }
    }
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
