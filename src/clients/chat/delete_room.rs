use super::proto;
use super::ChatClient;
use super::Room;
use crate::error::ConduitSdkError;

impl ChatClient {
    pub async fn delete_room(&self, room_id: String) -> Result<Room, ConduitSdkError> {
        let req = tonic::Request::new(proto::DeleteRoomRequest { id: room_id });
        let res = self
            ._client
            .clone()
            .delete_room(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}
