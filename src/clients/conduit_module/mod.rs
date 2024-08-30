pub(crate) mod proto {
    include!("../../../proto_gen/conduit.module.v1.rs");

    #[allow(unused)]
    pub(crate) use {
        admin_router_client::AdminRouterClient, client_router_client::ClientRouterClient,
        conduit_module_client::ConduitModuleClient, Request as RoutingRequest,
        Response as RoutingResponse, SocketRequest as RoutingSocketRequest,
        SocketResponse as RoutingSocketResponse,
    };
}

pub mod module;
pub mod routing;
