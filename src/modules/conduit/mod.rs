use crate::addr::GrpcAddress;
use crate::clients::{
    conduit::admin::AdminClient, conduit::config::ConfigClient, health::HealthClient,
};
use crate::error::ConduitSdkError;
use crate::sd::ServiceDiscoveryState;
use std::sync::Arc;
use tokio::sync::{watch, RwLock};
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

    /// Keeps track of the latest service discovery state.<br />
    /// Returns a background-synchronized `ServiceDiscoveryState` and a channel of `ServiceDiscoveryState` updates.
    pub async fn monitor_state(
        &self,
    ) -> Result<
        (
            Arc<RwLock<ServiceDiscoveryState>>,
            watch::Receiver<ServiceDiscoveryState>,
        ),
        ConduitSdkError,
    > {
        let initial_state = self.config_client.list_modules().await?;
        let (tx, rx) = watch::channel(initial_state.clone());
        let sd_state = Arc::new(RwLock::new(initial_state));
        let stream = self.config_client.watch_modules().await?;
        ServiceDiscoveryState::sync_from_stream(sd_state.clone(), stream, tx);
        Ok((sd_state, rx))
    }
}

impl std::fmt::Debug for Conduit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conduit @ {:?}", self.grpc_address)
    }
}
