use super::super::proto;
use super::super::AuthenticationClient;
use super::Team;
use crate::error::ConduitSdkError;

impl AuthenticationClient {
    pub async fn get_team(&self, team_id: String) -> Result<Option<Team>, ConduitSdkError> {
        let req = tonic::Request::new(proto::GetTeamRequest {
            team_id: Some(team_id),
        });
        let res: Team = self
            ._client
            .clone()
            .get_team(req)
            .await?
            .into_inner()
            .into();
        let res = res.get_team_res_workaround();
        Ok(res)
    }
}

impl Team {
    // Upstream seems to be returning semi-initialized objects for not-found teams...
    fn get_team_res_workaround(self) -> Option<Self> {
        match (self.name.trim().is_empty(), self.name.trim().is_empty()) {
            (false, false) => Some(self),
            _ => None,
        }
    }
}
