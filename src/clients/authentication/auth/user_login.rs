use super::super::proto;
use super::super::AuthenticationClient;
use crate::error::ConduitSdkError;
use crate::modules::router::SecurityClientId;

impl AuthenticationClient {
    pub async fn user_login(
        &self,
        user_id: String,
        security_client_id: SecurityClientId,
    ) -> Result<UserLoginResponse, ConduitSdkError> {
        let req = tonic::Request::new(proto::UserLoginRequest {
            user_id,
            client_id: security_client_id.into(),
        });
        let res = self
            ._client
            .clone()
            .user_login(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserLoginResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

impl From<proto::UserLoginResponse> for UserLoginResponse {
    fn from(grpc_res: proto::UserLoginResponse) -> Self {
        UserLoginResponse {
            access_token: grpc_res.access_token,
            refresh_token: grpc_res.refresh_token,
        }
    }
}
