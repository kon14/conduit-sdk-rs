use super::proto;
use super::PushNotificationsClient;
use crate::error::ConduitSdkError;
use crate::modules::push_notifications::PushNotificationToken;

impl PushNotificationsClient {
    pub async fn get_notification_tokens(
        &self,
        user_id: String,
    ) -> Result<Vec<PushNotificationToken>, ConduitSdkError> {
        let req = tonic::Request::new(proto::GetNotificationTokensRequest { user_id });
        let res = self
            ._client
            .clone()
            .get_notification_tokens(req)
            .await?
            .into_inner()
            .try_into()?;
        Ok(res)
    }
}

impl TryFrom<proto::GetNotificationTokensResponse> for Vec<PushNotificationToken> {
    type Error = ConduitSdkError;

    fn try_from(grpc_res: proto::GetNotificationTokensResponse) -> Result<Self, Self::Error> {
        grpc_res.token_documents
            .into_iter()
            .map(|token| {
                serde_json::from_str(&token).map_err(|err| ConduitSdkError::ApiBreakingChange(
                    format!("Invalid PushNotifications::GetNotificationTokensResponse::tokenDocuments value ({token})!"),
                    Some(Box::new(err)),
                ))
            })
            .collect()
    }
}
