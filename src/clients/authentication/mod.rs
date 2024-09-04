pub(crate) mod proto {
    include!("../../../proto_gen/authentication.rs");
}

use tonic::transport::Channel;

pub struct AuthenticationClient {
    pub(crate) _client: proto::authentication_client::AuthenticationClient<Channel>,
}

impl AuthenticationClient {
    pub fn new(channel: Channel) -> Self {
        AuthenticationClient {
            _client: proto::authentication_client::AuthenticationClient::new(channel),
        }
    }
}

mod auth;
mod team;
mod user;

pub use user::NewPassword;
