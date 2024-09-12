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

impl AuthorizationModule {
    pub fn new(grpc_address: GrpcAddress, channel: Channel) -> Self {
        let module_client = ConduitModuleClient::new(channel.clone());
        let service_client = AuthorizationClient::new(channel.clone());
        let health_client = HealthClient::new(channel);
        AuthorizationModule {
            grpc_address,
            _module_client: module_client,
            _service_client: service_client,
            health_client,
        }
    }
}

impl std::fmt::Debug for AuthorizationModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Authorization @ {:?}", self.grpc_address)
    }
}
