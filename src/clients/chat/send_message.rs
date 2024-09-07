use super::proto;
use super::ChatClient;
use crate::error::ConduitSdkError;

impl ChatClient {
    pub async fn send_message(
        &self,
        room_id: String,
        user_id: String,
        message: String,
    ) -> Result<(), ConduitSdkError> {
        let req = tonic::Request::new(proto::SendMessageRequest {
            room_id,
            user_id,
            message,
        });
        let res = self
            ._client
            .clone()
            .send_message(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}
