pub(crate) mod proto {
    include!("../../../proto_gen/sms.rs");

    pub(crate) use sms_client::SmsClient;
}
