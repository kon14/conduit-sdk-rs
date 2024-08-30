use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{conduit_module::proto::ConduitModuleClient, health::HealthClient};

pub struct UnknownModule {
    pub module_name: String,
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: Option<ConduitModuleClient<Channel>>,
    pub health_client: Option<HealthClient>,
}

impl std::fmt::Debug for UnknownModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {:?}", self.module_name, self.grpc_address)
    }
}
