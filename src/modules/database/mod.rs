use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    conduit_module::proto::ConduitModuleClient, database::proto::DatabaseClient,
    health::HealthClient,
};

pub struct DatabaseModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub(crate) _service_client: DatabaseClient<Channel>,
    pub health_client: HealthClient,
}

impl DatabaseModule {
    pub fn new(grpc_address: GrpcAddress, channel: Channel) -> Self {
        let module_client = ConduitModuleClient::new(channel.clone());
        let service_client = DatabaseClient::new(channel.clone());
        let health_client = HealthClient::new(channel);
        DatabaseModule {
            grpc_address,
            _module_client: module_client,
            _service_client: service_client,
            health_client,
        }
    }
}

impl std::fmt::Debug for DatabaseModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database @ {:?}", self.grpc_address)
    }
}
