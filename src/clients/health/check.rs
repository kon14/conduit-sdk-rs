use super::proto::HealthCheckRequest;
use super::{HealthCheckResponse, HealthClient};
use crate::error::ConduitSdkError;

impl HealthClient {
    pub async fn check(&self, service: String) -> Result<HealthCheckResponse, ConduitSdkError> {
        let req = HealthCheckRequest { service };
        let res = self
            ._client
            .clone()
            .check(req)
            .await?
            .into_inner()
            .try_into()?;
        Ok(res)
    }
}
