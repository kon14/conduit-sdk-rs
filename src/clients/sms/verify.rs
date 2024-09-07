use super::proto;
use super::SmsClient;
use crate::error::ConduitSdkError;

impl SmsClient {
    pub async fn verify(
        &self,
        verification_sid: String,
        code: String,
    ) -> Result<VerifyResponse, ConduitSdkError> {
        let req = tonic::Request::new(proto::VerifyRequest {
            verification_sid,
            code,
        });
        let res = self._client.clone().verify(req).await?.into_inner().into();
        Ok(res)
    }
}

pub struct VerifyResponse {
    pub verified: bool,
}

impl From<proto::VerifyResponse> for VerifyResponse {
    fn from(grpc_res: proto::VerifyResponse) -> Self {
        VerifyResponse {
            verified: grpc_res.verified,
        }
    }
}
