use super::proto;
use super::send_notification::SendNotificationRequest;
use super::PushNotificationsClient;
use crate::error::ConduitSdkError;

impl PushNotificationsClient {
    pub async fn send_many_notifications(
        &self,
        requests: Vec<SendNotificationRequest>,
    ) -> Result<(), ConduitSdkError> {
        let req = tonic::Request::new(requests.into());
        let res = self
            ._client
            .clone()
            .send_many_notifications(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}

impl From<Vec<SendNotificationRequest>> for proto::SendManyNotificationsRequest {
    fn from(sdk_req: Vec<SendNotificationRequest>) -> Self {
        proto::SendManyNotificationsRequest {
            notifications: sdk_req
                .into_iter()
                .map(|notification| notification.into())
                .collect(),
        }
    }
}
