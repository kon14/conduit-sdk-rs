use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    conduit_module::proto::ConduitModuleClient, health::HealthClient,
    push_notifications::PushNotificationsClient,
};

pub struct PushNotificationsModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub service_client: PushNotificationsClient,
    pub health_client: HealthClient,
}

impl PushNotificationsModule {
    pub fn new(grpc_address: GrpcAddress, channel: Channel) -> Self {
        let module_client = ConduitModuleClient::new(channel.clone());
        let service_client = PushNotificationsClient::new(channel.clone());
        let health_client = HealthClient::new(channel);
        PushNotificationsModule {
            grpc_address,
            _module_client: module_client,
            service_client,
            health_client,
        }
    }
}

impl std::fmt::Debug for PushNotificationsModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PushNotifications @ {:?}", self.grpc_address)
    }
}

mod token;

pub use token::{PushNotificationPlatform, PushNotificationToken};
