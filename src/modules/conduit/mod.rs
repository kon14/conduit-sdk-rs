use crate::addr::GrpcAddress;
use crate::clients::{
    conduit::admin::AdminClient, conduit::config::ConfigClient, health::HealthClient,
};
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
}

impl std::fmt::Debug for Conduit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conduit @ {:?}", self.grpc_address)
    }
}
