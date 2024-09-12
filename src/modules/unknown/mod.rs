use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{conduit_module::proto::ConduitModuleClient, health::HealthClient};

pub struct UnknownModule {
    pub module_name: String,
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: Option<ConduitModuleClient<Channel>>,
    pub health_client: Option<HealthClient>,
}

impl UnknownModule {
    pub fn new(module_name: String, grpc_address: GrpcAddress, channel: Channel) -> Self {
        // TODO:
        // There's currently no guarantee these services are going to be implemented.
        // Ideally service discovery should be aware of and expose this info.
        // Rolling with Some for now as merely instantiating the clients won't fail on lack of impl.
        let module_client = ConduitModuleClient::new(channel.clone());
        let health_client = HealthClient::new(channel);
        UnknownModule {
            module_name,
            grpc_address,
            _module_client: Some(module_client),
            health_client: Some(health_client),
        }
    }
}

impl std::fmt::Debug for UnknownModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {:?}", self.module_name, self.grpc_address)
    }
}
