use tonic::transport::Channel;

use super::proto;

pub struct AdminClient {
    pub(crate) _client: proto::admin_client::AdminClient<Channel>,
}

impl AdminClient {
    pub fn new(channel: Channel) -> Self {
        AdminClient {
            _client: proto::admin_client::AdminClient::new(channel),
        }
    }
}
