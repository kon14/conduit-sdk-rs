use super::proto;
use super::{PushNotificationsClient, PushNotificationsData};
use crate::error::ConduitSdkError;

impl PushNotificationsClient {
    pub async fn send_notification_to_many_devices(
        &self,
        request: SendNotificationToManyDevicesRequest,
    ) -> Result<(), ConduitSdkError> {
        let req = tonic::Request::new(request.into());
        let res = self
            ._client
            .clone()
            .send_notification_to_many_devices(req)
            .await?
            .into_inner()
            .into();
        Ok(res)
    }
}

#[derive(Debug, Clone)]
pub struct SendNotificationToManyDevicesRequest {
    pub receiver_user_ids: Vec<String>,
    pub data: PushNotificationsData,
    pub do_not_store: bool,
    pub is_silent: bool,
}

impl From<SendNotificationToManyDevicesRequest> for proto::SendNotificationToManyDevicesRequest {
    fn from(sdk_req: SendNotificationToManyDevicesRequest) -> Self {
        proto::SendNotificationToManyDevicesRequest {
            send_to: sdk_req.receiver_user_ids,
            title: sdk_req.data.title,
            platform: sdk_req
                .data
                .platform
                .map(|platform| serde_json::to_string(&platform).unwrap()),
            body: sdk_req.data.body,
            data: sdk_req
                .data
                .data
                .map(|data| serde_json::to_string(&data).unwrap()),
            do_not_store: sdk_req.do_not_store.then(|| true),
            is_silent: sdk_req.is_silent.then(|| true),
        }
    }
}
