use super::super::proto;
use super::super::AuthenticationClient;
use crate::error::ConduitSdkError;

impl AuthenticationClient {
    pub async fn remove_team_members(
        &self,
        team_id: String,
        member_ids: impl Into<Vec<String>>,
    ) -> Result<(), ConduitSdkError> {
        let req = tonic::Request::new(proto::ModifyTeamMembersRequest {
            team_id,
            member_ids: member_ids.into(),
        });
        let res = self
            ._client
            .clone()
            .remove_team_members(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}
