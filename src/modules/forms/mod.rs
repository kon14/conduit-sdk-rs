use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{conduit_module::proto::ConduitModuleClient, health::HealthClient};

pub struct FormsModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub health_client: HealthClient,
}

impl std::fmt::Debug for FormsModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Forms @ {:?}", self.grpc_address)
    }
}
