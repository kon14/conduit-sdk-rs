pub(crate) mod proto {
    include!("../../../proto_gen/chat.rs");

    pub(crate) use chat_client::ChatClient;
}
