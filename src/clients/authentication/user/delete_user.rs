use super::super::proto;
use super::super::AuthenticationClient;
use crate::error::ConduitSdkError;

impl AuthenticationClient {
    pub async fn delete_user(&self, user_id: String) -> Result<(), ConduitSdkError> {
        let req = tonic::Request::new(proto::UserDeleteRequest { user_id });
        let res = self
            ._client
            .clone()
            .user_delete(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}

impl From<proto::UserDeleteResponse> for () {
    fn from(_: proto::UserDeleteResponse) -> Self {
        ()
    }
}
