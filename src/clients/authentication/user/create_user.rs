use super::super::proto;
use super::super::AuthenticationClient;
use super::NewPassword;
use crate::error::ConduitSdkError;

impl AuthenticationClient {
    pub async fn create_user(
        &self,
        email: String,
        password: NewPassword,
        verified: bool,
    ) -> Result<CreateUserResponse, ConduitSdkError> {
        let password = match password {
            NewPassword::Key(pass) => Some(pass),
            NewPassword::Random => None,
        };
        let req = tonic::Request::new(proto::UserCreateRequest {
            email,
            password,
            verify: !verified,
        });
        let res = self
            ._client
            .clone()
            .user_create(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateUserResponse {
    pub password: Option<String>,
}

impl From<proto::UserCreateResponse> for CreateUserResponse {
    fn from(grpc_res: proto::UserCreateResponse) -> Self {
        CreateUserResponse {
            password: grpc_res.password,
        }
    }
}
