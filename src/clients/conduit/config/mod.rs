use tonic::transport::Channel;

use super::proto;

pub struct ConfigClient {
    pub(crate) _client: proto::config_client::ConfigClient<Channel>,
}

impl ConfigClient {
    pub fn new(channel: Channel) -> Self {
        ConfigClient {
            _client: proto::config_client::ConfigClient::new(channel),
        }
    }
}

mod list_modules;
mod watch_modules;

pub use watch_modules::WatchModulesStream;
