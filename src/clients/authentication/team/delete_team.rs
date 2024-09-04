use super::super::proto;
use super::super::AuthenticationClient;
use crate::error::ConduitSdkError;

impl AuthenticationClient {
    pub async fn delete_team(&self, team_id: String) -> Result<(), ConduitSdkError> {
        let req = tonic::Request::new(proto::TeamDeleteRequest { team_id });
        let res = self
            ._client
            .clone()
            .team_delete(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}

impl From<proto::TeamDeleteResponse> for () {
    fn from(_: proto::TeamDeleteResponse) -> Self {
        ()
    }
}
