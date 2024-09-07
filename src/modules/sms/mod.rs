use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    conduit_module::proto::ConduitModuleClient, health::HealthClient, sms::SmsClient,
};

pub struct SmsModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub service_client: SmsClient,
    pub health_client: HealthClient,
}

impl SmsModule {
    pub fn new(grpc_address: GrpcAddress, channel: Channel) -> Self {
        let module_client = ConduitModuleClient::new(channel.clone());
        let service_client = SmsClient::new(channel.clone());
        let health_client = HealthClient::new(channel);
        SmsModule {
            grpc_address,
            _module_client: module_client,
            service_client,
            health_client,
        }
    }
}

impl std::fmt::Debug for SmsModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SMS @ {:?}", self.grpc_address)
    }
}
