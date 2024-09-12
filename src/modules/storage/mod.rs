use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    conduit_module::proto::ConduitModuleClient, health::HealthClient, storage::StorageClient,
};

pub struct StorageModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub service_client: StorageClient,
    pub health_client: HealthClient,
}

impl StorageModule {
    pub fn new(grpc_address: GrpcAddress, channel: Channel) -> Self {
        let module_client = ConduitModuleClient::new(channel.clone());
        let service_client = StorageClient::new(channel.clone());
        let health_client = HealthClient::new(channel);
        StorageModule {
            grpc_address,
            _module_client: module_client,
            service_client,
            health_client,
        }
    }
}

impl std::fmt::Debug for StorageModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Storage @ {:?}", self.grpc_address)
    }
}
