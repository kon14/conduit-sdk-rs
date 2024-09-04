pub(crate) mod proto {
    include!("../../../proto_gen/grpc.health.v1.rs");
}

use crate::error::ConduitSdkError;
use tonic::transport::Channel;

pub struct HealthClient {
    pub(crate) _client: proto::health_client::HealthClient<Channel>,
}

impl HealthClient {
    pub fn new(channel: Channel) -> Self {
        HealthClient {
            _client: proto::health_client::HealthClient::new(channel),
        }
    }
}

impl ServingStatus {
    fn try_from_i32_cnd(grpc_status: i32) -> Result<Self, ConduitSdkError> {
        ServingStatus::try_from(grpc_status).map_err(|err| {
            ConduitSdkError::ApiBreakingChange(
                format!("Invalid health check status value ({grpc_status})!"),
                Some(Box::new(err)),
            )
        })
    }
}

#[derive(Debug)]
pub struct HealthCheckResponse {
    pub status: ServingStatus,
}

impl TryFrom<proto::HealthCheckResponse> for HealthCheckResponse {
    type Error = ConduitSdkError;

    fn try_from(grpc_res: proto::HealthCheckResponse) -> Result<Self, Self::Error> {
        let status: ServingStatus = ServingStatus::try_from_i32_cnd(grpc_res.status)?;
        Ok(HealthCheckResponse { status })
    }
}

mod check;
mod watch;

pub use watch::HealthWatchStream;

pub use proto::health_check_response::ServingStatus;
