use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    chat::proto::ChatClient, conduit_module::proto::ConduitModuleClient, health::HealthClient,
};

pub struct ChatModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub(crate) _service_client: ChatClient<Channel>,
    pub health_client: HealthClient,
}

impl std::fmt::Debug for ChatModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chat @ {:?}", self.grpc_address)
    }
}
