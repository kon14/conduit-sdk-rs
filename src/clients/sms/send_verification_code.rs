use super::proto;
use super::SmsClient;
use crate::error::ConduitSdkError;

impl SmsClient {
    pub async fn send_verification_code(
        &self,
        to: String,
    ) -> Result<SendVerificationCodeResponse, ConduitSdkError> {
        let req = tonic::Request::new(proto::SendVerificationCodeRequest { to });
        let res = self
            ._client
            .clone()
            .send_verification_code(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}

pub struct SendVerificationCodeResponse {
    pub verification_sid: String,
}

impl From<proto::SendVerificationCodeResponse> for SendVerificationCodeResponse {
    fn from(grpc_res: proto::SendVerificationCodeResponse) -> Self {
        SendVerificationCodeResponse {
            verification_sid: grpc_res.verification_sid,
        }
    }
}
