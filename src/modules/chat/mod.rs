use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    chat::ChatClient, conduit_module::proto::ConduitModuleClient, health::HealthClient,
};

pub struct ChatModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub service_client: ChatClient,
    pub health_client: HealthClient,
}

impl ChatModule {
    pub fn new(grpc_address: GrpcAddress, channel: Channel) -> Self {
        let module_client = ConduitModuleClient::new(channel.clone());
        let service_client = ChatClient::new(channel.clone());
        let health_client = HealthClient::new(channel);
        ChatModule {
            grpc_address,
            _module_client: module_client,
            service_client,
            health_client,
        }
    }
}

impl std::fmt::Debug for ChatModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chat @ {:?}", self.grpc_address)
    }
}

mod room;

pub use room::Room;
