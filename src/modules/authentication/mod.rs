use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    authentication::AuthenticationClient, conduit_module::proto::ConduitModuleClient,
    health::HealthClient,
};

pub struct AuthenticationModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub service_client: AuthenticationClient,
    pub health_client: HealthClient,
}

impl AuthenticationModule {
    pub fn new(grpc_address: GrpcAddress, channel: Channel) -> Self {
        let module_client = ConduitModuleClient::new(channel.clone());
        let service_client = AuthenticationClient::new(channel.clone());
        let health_client = HealthClient::new(channel);
        AuthenticationModule {
            grpc_address,
            _module_client: module_client,
            service_client,
            health_client,
        }
    }
}

impl std::fmt::Debug for AuthenticationModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Authentication @ {:?}", self.grpc_address)
    }
}

mod path;
mod team;

pub use path::AuthenticationPath;
pub use team::Team;
