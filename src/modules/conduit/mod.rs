use crate::addr::GrpcAddress;
use crate::clients::{
    conduit::admin::AdminClient, conduit::config::ConfigClient, health::HealthClient,
};
use crate::error::ConduitSdkError;
use crate::sd::{sync_sd_state, ServiceDiscoveryState};
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::Channel;

pub struct Conduit {
    pub grpc_address: GrpcAddress,
    pub config_client: ConfigClient,
    pub admin_client: AdminClient,
    pub health_client: HealthClient,
}

impl Conduit {
    pub fn new(grpc_address: GrpcAddress, channel: Channel) -> Self {
        let config_client = ConfigClient::new(channel.clone());
        let admin_client = AdminClient::new(channel.clone());
        let health_client = HealthClient::new(channel);
        Conduit {
            grpc_address,
            config_client,
            admin_client,
            health_client,
        }
    }

    pub async fn monitor_modules(
        &self,
    ) -> Result<Arc<RwLock<ServiceDiscoveryState>>, ConduitSdkError> {
        let initial_state = self.config_client.list_modules().await?;
        let sd_state = Arc::new(RwLock::new(initial_state));
        let stream = self.config_client.watch_modules().await?;
        sync_sd_state(sd_state.clone(), stream);
        Ok(sd_state)
    }
}

impl std::fmt::Debug for Conduit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conduit @ {:?}", self.grpc_address)
    }
}
