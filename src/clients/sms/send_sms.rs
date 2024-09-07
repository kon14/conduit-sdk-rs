use super::proto;
use super::SmsClient;
use crate::error::ConduitSdkError;

impl SmsClient {
    pub async fn send_sms(
        &self,
        to: String,
        message: String,
    ) -> Result<SendSmsResponse, ConduitSdkError> {
        let req = tonic::Request::new(proto::SendSmsRequest { to, message });
        let res = self
            ._client
            .clone()
            .send_sms(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}

pub struct SendSmsResponse {
    pub message: String,
}

impl From<proto::SendSmsResponse> for SendSmsResponse {
    fn from(grpc_res: proto::SendSmsResponse) -> Self {
        SendSmsResponse {
            message: grpc_res.message,
        }
    }
}
