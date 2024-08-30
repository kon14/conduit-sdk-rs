use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    conduit_module::proto::ConduitModuleClient, email::proto::EmailClient, health::HealthClient,
};

pub struct EmailModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub(crate) _service_client: EmailClient<Channel>,
    pub health_client: HealthClient,
}

impl std::fmt::Debug for EmailModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email @ {:?}", self.grpc_address)
    }
}
