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

impl std::fmt::Debug for RouterModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Router @ {:?}", self.grpc_address)
    }
}
