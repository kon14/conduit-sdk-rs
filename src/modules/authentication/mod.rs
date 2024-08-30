use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    authentication::proto::AuthenticationClient, conduit_module::proto::ConduitModuleClient,
    health::HealthClient,
};

pub struct AuthenticationModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub(crate) _service_client: AuthenticationClient<Channel>,
    pub health_client: HealthClient,
}

impl std::fmt::Debug for AuthenticationModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Authentication @ {:?}", self.grpc_address)
    }
}
