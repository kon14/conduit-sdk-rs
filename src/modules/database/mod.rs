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

impl std::fmt::Debug for DatabaseModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database @ {:?}", self.grpc_address)
    }
}
