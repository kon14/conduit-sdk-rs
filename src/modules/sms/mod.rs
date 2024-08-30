use tonic::transport::Channel;

use crate::addr::GrpcAddress;
use crate::clients::{
    conduit_module::proto::ConduitModuleClient, health::HealthClient, sms::proto::SmsClient,
};

pub struct SmsModule {
    pub grpc_address: GrpcAddress,
    pub(crate) _module_client: ConduitModuleClient<Channel>,
    pub(crate) _service_client: SmsClient<Channel>,
    pub health_client: HealthClient,
}

impl std::fmt::Debug for SmsModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SMS @ {:?}", self.grpc_address)
    }
}
