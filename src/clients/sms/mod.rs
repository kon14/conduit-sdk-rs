pub(crate) mod proto {
    include!("../../../proto_gen/sms.rs");
}

use tonic::transport::Channel;

pub struct SmsClient {
    pub(crate) _client: proto::sms_client::SmsClient<Channel>,
}
impl SmsClient {
    pub fn new(channel: Channel) -> Self {
        SmsClient {
            _client: proto::sms_client::SmsClient::new(channel),
        }
    }
}

mod send_sms;
mod send_verification_code;
mod verify;

pub use send_sms::SendSmsResponse;
pub use send_verification_code::SendVerificationCodeResponse;
pub use verify::VerifyResponse;
