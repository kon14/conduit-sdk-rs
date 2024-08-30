use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum ConduitSdkError {
    #[error("ConduitSdkError: gRPC request failed ({0})")]
    GrpcStatus(Status),

    /// Typically indicates a breaking change in Conduit's gRPC API
    /// or an SDK bug related to response handling.
    #[error("ConduitSdkError: API Breaking Change - {0}")]
    ApiBreakingChange(String, Option<Box<dyn std::error::Error + Send + Sync>>),

    #[error("ConduitSdkError: Invalid env var value ({0})")]
    InvalidEnvValue(String),

    #[error("ConduitSdkError: Connection failure - {0}")]
    ConnectionFailure(String, Option<Box<dyn std::error::Error + Send + Sync>>),
}

impl From<Status> for ConduitSdkError {
    fn from(tonic: Status) -> Self {
        ConduitSdkError::GrpcStatus(tonic)
    }
}
