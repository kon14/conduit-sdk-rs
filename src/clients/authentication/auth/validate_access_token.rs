use super::super::proto;
use super::super::AuthenticationClient;
use crate::error::ConduitSdkError;
use crate::modules::authentication::AuthenticationPath;

impl AuthenticationClient {
    pub async fn validate_access_token(
        &self,
        access_token: String,
        path: Option<AuthenticationPath>,
    ) -> Result<ValidateAccessTokenResponse, ConduitSdkError> {
        let req = tonic::Request::new(proto::ValidateAccessTokenRequest {
            access_token,
            path: path.map(Into::into),
        });
        self._client
            .clone()
            .validate_access_token(req)
            .await?
            .into_inner()
            .try_into()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValidateAccessTokenStatus {
    Authenticated = 0,
    Unauthenticated = 1,
    Requires2FA = 2,
    UserBlocked = 3,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValidateAccessTokenResponse {
    pub status: ValidateAccessTokenStatus,
    pub user_id: String,
}

impl ValidateAccessTokenStatus {
    fn try_from_i32_cnd(grpc_status: i32) -> Result<Self, ConduitSdkError> {
        let proto_status = proto::validate_access_token_response::Status::try_from(grpc_status).map_err(|err| {
            ConduitSdkError::ApiBreakingChange(
                format!("Invalid Authentication::ValidateAccessTokenStatus::Status value ({grpc_status})!"),
                Some(Box::new(err)),
            )
        })?;
        let status = match proto_status {
            proto::validate_access_token_response::Status::Authenticated => {
                ValidateAccessTokenStatus::Authenticated
            }
            proto::validate_access_token_response::Status::Unauthenticated => {
                ValidateAccessTokenStatus::Unauthenticated
            }
            proto::validate_access_token_response::Status::Requires2fa => {
                ValidateAccessTokenStatus::Requires2FA
            }
            proto::validate_access_token_response::Status::UserBlocked => {
                ValidateAccessTokenStatus::UserBlocked
            }
        };
        Ok(status)
    }
}

impl TryFrom<proto::ValidateAccessTokenResponse> for ValidateAccessTokenResponse {
    type Error = ConduitSdkError;

    fn try_from(grpc_res: proto::ValidateAccessTokenResponse) -> Result<Self, Self::Error> {
        let status = ValidateAccessTokenStatus::try_from_i32_cnd(grpc_res.status)?;
        let user_id = grpc_res.user_id.ok_or(ConduitSdkError::ApiBreakingChange(
            "Missing user_id!".to_string(),
            None,
        ))?;
        Ok(ValidateAccessTokenResponse { status, user_id })
    }
}
