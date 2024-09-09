use super::nodemailer::Attachment;
use super::proto;
use super::EmailClient;
use crate::error::ConduitSdkError;
use std::collections::HashMap;

impl EmailClient {
    pub async fn send_email(
        &self,
        request: SendEmailRequest,
    ) -> Result<SendEmailResponse, ConduitSdkError> {
        let req = tonic::Request::new(request.into());
        let res = self
            ._client
            .clone()
            .send_email(req)
            .await?
            .into_inner()
            .try_into()?;
        Ok(res)
    }
}

pub struct SendEmailParams {
    pub email: String,
    pub variables: Option<HashMap<String, String>>,
    pub sender: Option<String>,
    pub cc: Option<Vec<String>>,
    pub reply_to: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
}

pub struct SendEmailRequest {
    pub template_name: String,
    pub params: SendEmailParams,
}

pub struct SendEmailResponse {
    pub sent_message_info_json: serde_json::Value,
}

impl From<SendEmailParams> for proto::send_email_request::SendEmailParams {
    fn from(sdk_params: SendEmailParams) -> Self {
        proto::send_email_request::SendEmailParams {
            email: sdk_params.email,
            variables: serde_json::to_string(&sdk_params.variables).unwrap(),
            sender: sdk_params.sender,
            cc: sdk_params.cc.unwrap_or_default(),
            reply_to: sdk_params.reply_to,
            attachments: sdk_params
                .attachments
                .unwrap_or_default()
                .into_iter()
                .map(|attachment| serde_json::to_string(&attachment).unwrap())
                .collect(),
        }
    }
}

impl From<SendEmailRequest> for proto::SendEmailRequest {
    fn from(sdk_req: SendEmailRequest) -> Self {
        proto::SendEmailRequest {
            template_name: sdk_req.template_name,
            params: Some(sdk_req.params.into()),
        }
    }
}

impl TryFrom<proto::SendEmailResponse> for SendEmailResponse {
    type Error = ConduitSdkError;

    fn try_from(grpc_res: proto::SendEmailResponse) -> Result<Self, ConduitSdkError> {
        let sent_message_info_str = grpc_res.sent_message_info;
        let sent_message_info_json: serde_json::Value = serde_json::from_str(sent_message_info_str.as_str())
            .map_err(|err| {
                ConduitSdkError::ApiBreakingChange(
                    format!("Invalid Email::SendEmailResponse message value. Unparsable sent_message_info JSON ({sent_message_info_str})!"),
                    Some(Box::new(err)),
                )
            })?;
        Ok(SendEmailResponse {
            sent_message_info_json,
        })
    }
}
