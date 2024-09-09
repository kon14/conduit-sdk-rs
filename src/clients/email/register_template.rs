use super::proto;
use super::EmailClient;
use crate::error::ConduitSdkError;

impl EmailClient {
    pub async fn register_template(
        &self,
        request: RegisterTemplateRequest,
    ) -> Result<RegisterTemplateResponse, ConduitSdkError> {
        let req = tonic::Request::new(request.into());
        let res = self
            ._client
            .clone()
            .register_template(req)
            .await?
            .into_inner()
            .try_into()?;
        Ok(res)
    }
}

pub struct RegisterTemplateRequest {
    pub name: String,
    pub subject: String,
    pub body: String,
    pub variables: Option<Vec<String>>,
    pub sender: Option<String>,
}

impl From<RegisterTemplateRequest> for proto::RegisterTemplateRequest {
    fn from(sdk_req: RegisterTemplateRequest) -> Self {
        proto::RegisterTemplateRequest {
            name: sdk_req.name,
            subject: sdk_req.subject,
            body: sdk_req.body,
            variables: sdk_req.variables.unwrap_or_default(),
            sender: sdk_req.sender,
        }
    }
}

pub struct RegisterTemplateResponse {
    pub template_json: serde_json::Value,
}

impl TryFrom<proto::RegisterTemplateResponse> for RegisterTemplateResponse {
    type Error = ConduitSdkError;

    fn try_from(grpc_res: proto::RegisterTemplateResponse) -> Result<Self, ConduitSdkError> {
        let template_str = grpc_res.template;
        let template_json: serde_json::Value = serde_json::from_str(template_str.as_str())
            .map_err(|err| {
                ConduitSdkError::ApiBreakingChange(
                    format!("Invalid Email::RegisterTemplateResponse message value. Unparsable template JSON ({template_str})!"),
                    Some(Box::new(err)),
                )
            })?;
        Ok(RegisterTemplateResponse { template_json })
    }
}
