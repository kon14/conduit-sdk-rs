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

// #[derive(
//     Clone,
//     Copy,
//     Debug,
//     PartialEq,
//     Eq,
//     Hash,
//     PartialOrd,
//     Ord,
// )]
// #[repr(i32)]
// pub enum ServingStatus {
//     Unknown = 0,
//     Serving = 1,
//     NotServing = 2,
//     /// Used only by the Watch method.
//     ServiceUnknown = 3,
// }

// impl From<proto::health_check_response::ServingStatus> for ServingStatus {
//     fn from(proto_status: proto::health_check_response::ServingStatus) -> Self {
//         match proto_status {
//             proto::health_check_response::ServingStatus::Unknown => ServingStatus::Unknown,
//             proto::health_check_response::ServingStatus::Serving => ServingStatus::Serving,
//             proto::health_check_response::ServingStatus::NotServing => ServingStatus::NotServing,
//             proto::health_check_response::ServingStatus::ServiceUnknown => ServingStatus::ServiceUnknown,
//         }
//     }
// }
//
// impl From<ServingStatus> for proto::health_check_response::ServingStatus {
//     fn from(cnd_status: ServingStatus) -> Self {
//         match cnd_status {
//             ServingStatus::Unknown => proto::health_check_response::ServingStatus::Unknown,
//             ServingStatus::Serving => proto::health_check_response::ServingStatus::Serving,
//             ServingStatus::NotServing => proto::health_check_response::ServingStatus::NotServing,
//             ServingStatus::ServiceUnknown => proto::health_check_response::ServingStatus::ServiceUnknown,
//         }
//     }
// }

impl ServingStatus {
    fn try_from_i32_cnd(grpc_status: i32) -> Result<Self, ConduitSdkError> {
        ServingStatus::try_from(grpc_status).map_err(|err| {
            ConduitSdkError::ApiBreakingChange(
                format!("Invalid health check status value ({})!", grpc_status),
                Some(Box::new(err)),
            )
        })
    }
}

#[derive(Debug)]
pub struct HealthCheckResponse {
    pub status: ServingStatus,
}

// impl TryFrom<i32> for ServingStatus {
//     type Error = ConduitSdkError;
//
//     fn try_from(grpc_status: i32) -> Result<Self, Self::Error> {
//         ServingStatus::try_from(grpc_status).map_err(|err| {
//             ConduitSdkError::ApiBreakingChange(
//                 format!("Invalid health check status value ({})!", grpc_status),
//                 Some(Box::new(err)),
//             )
//         })
//     }
// }

impl TryFrom<proto::HealthCheckResponse> for HealthCheckResponse {
    type Error = ConduitSdkError;

    fn try_from(grpc_res: proto::HealthCheckResponse) -> Result<Self, Self::Error> {
        // let status: ServingStatus = grpc_res.status.try_into()?;
        let status: ServingStatus = ServingStatus::try_from_i32_cnd(grpc_res.status)?;
        Ok(HealthCheckResponse { status })
    }
}

mod check;
mod watch;

pub use check::*;
pub use watch::*;

pub use proto::health_check_response::ServingStatus;
