use super::proto;
use super::PushNotificationsClient;
use crate::error::ConduitSdkError;
use crate::modules::push_notifications::{PushNotificationPlatform, PushNotificationToken};

impl PushNotificationsClient {
    pub async fn set_notification_token(
        &self,
        user_id: String,
        token: String,
        platform: PushNotificationPlatform,
    ) -> Result<PushNotificationToken, ConduitSdkError> {
        let req = tonic::Request::new(proto::SetNotificationTokenRequest {
            user_id,
            token,
            platform: serde_json::to_string(&platform).unwrap(),
        });
        let res = self
            ._client
            .clone()
            .set_notification_token(req)
            .await?
            .into_inner()
            .try_into()?;
        Ok(res)
    }
}

impl TryFrom<proto::SetNotificationTokenResponse> for PushNotificationToken {
    type Error = ConduitSdkError;

    fn try_from(grpc_res: proto::SetNotificationTokenResponse) -> Result<Self, Self::Error> {
        let token = grpc_res.new_token_document;
        serde_json::from_str(&token)
            .map_err(|err| ConduitSdkError::ApiBreakingChange(
                format!("Invalid PushNotifications::SetNotificationTokensResponse::newTokenDocument value ({token})!"),
                Some(Box::new(err)),
            ))
    }
}
