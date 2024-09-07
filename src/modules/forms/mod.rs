use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{conduit_module::proto::ConduitModuleClient, health::HealthClient};

pub struct FormsModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub health_client: HealthClient,
}

impl FormsModule {
    pub fn new(grpc_address: GrpcAddress, channel: Channel) -> Self {
        let module_client = ConduitModuleClient::new(channel.clone());
        let health_client = HealthClient::new(channel);
        FormsModule {
            grpc_address,
            _module_client: module_client,
            health_client,
        }
    }
}

impl std::fmt::Debug for FormsModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Forms @ {:?}", self.grpc_address)
    }
}
