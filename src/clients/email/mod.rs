pub(crate) mod proto {
    include!("../../../proto_gen/email.rs");
}

use tonic::transport::Channel;

pub struct EmailClient {
    pub(crate) _client: proto::email_client::EmailClient<Channel>,
}

impl EmailClient {
    pub fn new(channel: Channel) -> Self {
        EmailClient {
            _client: proto::email_client::EmailClient::new(channel),
        }
    }
}

mod nodemailer;
pub use nodemailer::*;

mod register_template;
mod send_email;

pub use register_template::{RegisterTemplateRequest, RegisterTemplateResponse};
pub use send_email::{SendEmailParams, SendEmailRequest, SendEmailResponse};
