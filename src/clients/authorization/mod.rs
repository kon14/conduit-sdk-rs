pub(crate) mod proto {
    include!("../../../proto_gen/authorization.rs");

    pub(crate) use authorization_client::AuthorizationClient;
}
