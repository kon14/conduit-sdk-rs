pub(crate) mod proto {
    include!("../../../proto_gen/chat.rs");
}

use crate::modules::chat::Room;
use tonic::transport::Channel;

pub struct ChatClient {
    pub(crate) _client: proto::chat_client::ChatClient<Channel>,
}

impl ChatClient {
    pub fn new(channel: Channel) -> Self {
        ChatClient {
            _client: proto::chat_client::ChatClient::new(channel),
        }
    }
}

impl From<proto::Room> for Room {
    fn from(grpc_res: proto::Room) -> Self {
        Room {
            id: grpc_res.id,
            name: grpc_res.name,
            participant_ids: grpc_res.participants,
        }
    }
}

mod create_room;
mod delete_room;
mod send_message;
