use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    authorization::proto::AuthorizationClient, conduit_module::proto::ConduitModuleClient,
    health::HealthClient,
};

pub struct AuthorizationModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub(crate) _service_client: AuthorizationClient<Channel>,
    pub health_client: HealthClient,
}

impl std::fmt::Debug for AuthorizationModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Authorization @ {:?}", self.grpc_address)
    }
}
