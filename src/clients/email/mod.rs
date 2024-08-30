pub(crate) mod proto {
    include!("../../../proto_gen/email.rs");

    pub(crate) use email_client::EmailClient;
}
