use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    conduit_module::proto::ConduitModuleClient, health::HealthClient, storage::proto::StorageClient,
};

pub struct StorageModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub(crate) _service_client: StorageClient<Channel>,
    pub health_client: HealthClient,
}

impl std::fmt::Debug for StorageModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Storage @ {:?}", self.grpc_address)
    }
}
