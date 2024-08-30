pub(crate) mod proto {
    include!("../../../proto_gen/database.rs");

    pub(crate) use database_provider_client::DatabaseProviderClient as DatabaseClient;
}
