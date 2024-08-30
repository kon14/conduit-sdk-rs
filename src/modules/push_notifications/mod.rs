use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    conduit_module::proto::ConduitModuleClient, health::HealthClient,
    push_notifications::proto::PushNotificationsClient,
};

pub struct PushNotificationsModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub(crate) _service_client: PushNotificationsClient<Channel>,
    pub health_client: HealthClient,
}

impl std::fmt::Debug for PushNotificationsModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PushNotifications @ {:?}", self.grpc_address)
    }
}
