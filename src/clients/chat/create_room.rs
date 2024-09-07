use super::proto;
use super::ChatClient;
use super::Room;
use crate::error::ConduitSdkError;

impl ChatClient {
    pub async fn create_room(
        &self,
        name: String,
        participant_ids: impl Into<Vec<String>>,
    ) -> Result<Room, ConduitSdkError> {
        let req = tonic::Request::new(proto::CreateRoomRequest {
            name,
            participants: participant_ids.into(),
        });
        let res = self
            ._client
            .clone()
            .create_room(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}
