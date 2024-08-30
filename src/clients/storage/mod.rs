pub(crate) mod proto {
    include!("../../../proto_gen/storage.rs");

    pub(crate) use storage_client::StorageClient;
}
