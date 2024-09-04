use super::super::proto;
use super::super::AuthenticationClient;
use super::NewPassword;
use crate::error::ConduitSdkError;

impl AuthenticationClient {
    pub async fn change_user_pass(
        &self,
        email: String,
        password: NewPassword,
    ) -> Result<ChangeUserPasswordResponse, ConduitSdkError> {
        let password = match password {
            NewPassword::Key(pass) => Some(pass),
            NewPassword::Random => None,
        };
        let req = tonic::Request::new(proto::UserChangePass { email, password });
        let res = self
            ._client
            .clone()
            .change_pass(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChangeUserPasswordResponse {
    pub password: Option<String>,
}

impl From<proto::UserCreateResponse> for ChangeUserPasswordResponse {
    fn from(grpc_res: proto::UserCreateResponse) -> Self {
        ChangeUserPasswordResponse {
            password: grpc_res.password,
        }
    }
}
