pub(crate) mod proto {
    include!("../../../proto_gen/router.rs");

    pub(crate) use router_client::RouterClient;
}
