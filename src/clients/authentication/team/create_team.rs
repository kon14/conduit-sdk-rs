use super::super::proto;
use super::super::AuthenticationClient;
use super::Team;
use crate::error::ConduitSdkError;

impl AuthenticationClient {
    pub async fn create_team(
        &self,
        name: String,
        parent_team: Option<String>,
        is_default: bool,
    ) -> Result<Team, ConduitSdkError> {
        let req = tonic::Request::new(proto::CreateTeamRequest {
            name,
            parent_team,
            is_default: Some(is_default),
        });
        let res = self
            ._client
            .clone()
            .create_team(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}
