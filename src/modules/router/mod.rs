use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    conduit_module::proto::ConduitModuleClient, health::HealthClient, router::proto::RouterClient,
};

pub struct RouterModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub(crate) _service_client: RouterClient<Channel>,
    pub health_client: HealthClient,
}

impl RouterModule {
    pub fn new(grpc_address: GrpcAddress, channel: Channel) -> Self {
        let module_client = ConduitModuleClient::new(channel.clone());
        let service_client = RouterClient::new(channel.clone());
        let health_client = HealthClient::new(channel);
        RouterModule {
            grpc_address,
            _module_client: module_client,
            _service_client: service_client,
            health_client,
        }
    }
}

impl std::fmt::Debug for RouterModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Router @ {:?}", self.grpc_address)
    }
}

mod security_client;

pub use security_client::*;
