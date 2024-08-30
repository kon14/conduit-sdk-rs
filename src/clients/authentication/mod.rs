pub(crate) mod proto {
    include!("../../../proto_gen/authentication.rs");

    pub(crate) use authentication_client::AuthenticationClient;
}
