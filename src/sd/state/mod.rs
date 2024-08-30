use super::module_name::ModuleName;
use crate::addr::GrpcAddress;
use crate::clients::conduit::config::WatchModulesStream;
use chrono::{DateTime, Utc};
use futures::Stream;
use log::error;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::task::Context;

/// A representation of a module's remotely tracked distributed service discovery state.
#[derive(Clone, Eq, PartialEq)]
pub struct ServiceDiscoveryModuleState {
    pub module_name: ModuleName,
    pub grpc_address: GrpcAddress,
    pub is_serving: bool,
}

/// A representation of the platform's remotely tracked distributed service discovery state.
#[derive(Clone, Eq)]
pub struct ServiceDiscoveryState {
    pub module_states: HashMap<ModuleName, ServiceDiscoveryModuleState>,
    pub last_updated: DateTime<Utc>,
}

pub fn sync_sd_state(sd_state: Arc<RwLock<ServiceDiscoveryState>>, stream: WatchModulesStream) {
    tokio::spawn(async move {
        let mut cx = Context::from_waker(futures::task::noop_waker_ref());
        let mut pinned = std::pin::pin!(stream);
        loop {
            match pinned.as_mut().poll_next(&mut cx) {
                std::task::Poll::Ready(Some(state)) => {
                    match state {
                        Ok(state) => {
                            let mut writable = sd_state.write().unwrap();
                            *writable = state;
                        }
                        Err(err) => {
                            error!("{err}");
                        }
                    };
                }
                std::task::Poll::Ready(None) => break,
                _ => {}
            }
        }
    });
}

impl std::fmt::Debug for ServiceDiscoveryModuleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serving_str = if self.is_serving {
            "Serving"
        } else {
            "Not Serving"
        };
        write!(
            f,
            "{:?} @ {:?} - [{}]",
            self.module_name, self.grpc_address, serving_str
        )
    }
}

impl std::fmt::Debug for ServiceDiscoveryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            writeln!(f, "ServiceDiscoveryState {{")?;
            writeln!(f, "    module_states: {{")?;
            for (module_name, module_state) in &self.module_states {
                writeln!(
                    f,
                    "        {:?}: {{ grpc_address: {:?}, is_serving: {} }},",
                    module_name, module_state.grpc_address, module_state.is_serving
                )?;
            }
            writeln!(f, "    }},")?;
            writeln!(f, "    last_updated: {:?}", self.last_updated)?;
            writeln!(f, "}}")
        } else {
            write!(
                f,
                "ServiceDiscoveryState {{ module_states: {:?}, last_updated: {:?} }}",
                self.module_states, self.last_updated
            )
        }
    }
}

impl PartialEq for ServiceDiscoveryState {
    fn eq(&self, other: &Self) -> bool {
        self.module_states == other.module_states
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::addr::GrpcHost;
    use std::net::{IpAddr, Ipv4Addr};

    /// Tests `ServiceDiscoveryState`'s `PartialEq` implementation.
    ///
    /// More importantly, it asserts that said impl gets updated should new struct fields be introduced.
    #[test]
    fn test_service_discovery_state_partial_eq() {
        let state_a = ServiceDiscoveryState {
            module_states: vec![
                (
                    ModuleName::Authentication,
                    ServiceDiscoveryModuleState {
                        module_name: ModuleName::Authentication,
                        grpc_address: GrpcAddress {
                            host: GrpcHost::Ip(IpAddr::from(Ipv4Addr::new(0, 0, 0, 0))),
                            port: 55160,
                        },
                        is_serving: true,
                    },
                ),
                (
                    ModuleName::Database,
                    ServiceDiscoveryModuleState {
                        module_name: ModuleName::Database,
                        grpc_address: GrpcAddress {
                            host: GrpcHost::Ip(IpAddr::from(Ipv4Addr::new(0, 0, 0, 0))),
                            port: 55161,
                        },
                        is_serving: true,
                    },
                ),
            ]
            .into_iter()
            .collect(),
            last_updated: Utc::now(),
        };
        let mut state_b = state_a.clone();
        state_b.last_updated = Utc::now();

        assert_ne!(state_a.last_updated, state_b.last_updated);
        assert_eq!(
            state_a, state_b,
            "ServiceDiscoveryState introduced new fields, breaking PartialEq impl!",
        );
    }
}
