pub(crate) mod proto {
    include!("../../../proto_gen/pushnotifications.rs");

    pub(crate) use push_notifications_client::PushNotificationsClient;
}
