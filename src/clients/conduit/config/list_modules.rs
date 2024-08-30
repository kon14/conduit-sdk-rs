use super::proto::{module_list_response::ModuleResponse, ModuleListResponse};
use super::ConfigClient;
use crate::addr::url_remap::ModuleUrlMappings;
use crate::error::ConduitSdkError;
use crate::sd::{ModuleName, ServiceDiscoveryModuleState, ServiceDiscoveryState};
use chrono::Utc;
use std::collections::HashMap;

impl ConfigClient {
    pub async fn list_modules(&self) -> Result<ServiceDiscoveryState, ConduitSdkError> {
        let req = tonic::Request::new(());
        self._client
            .clone()
            .module_list(req)
            .await?
            .into_inner()
            .try_into()
    }
}

impl TryFrom<ModuleResponse> for ServiceDiscoveryModuleState {
    type Error = ConduitSdkError;

    fn try_from(grpc: ModuleResponse) -> Result<Self, Self::Error> {
        let grpc_address = grpc.url.parse().map_err(|err| {
            ConduitSdkError::ApiBreakingChange(
                "Invalid Config::ModuleResponse response message value. Unparsable modules[].url!"
                    .to_string(),
                Some(Box::new(err)),
            )
        })?;
        Ok(ServiceDiscoveryModuleState {
            module_name: grpc.module_name.as_str().into(),
            grpc_address,
            is_serving: grpc.serving,
        })
    }
}

impl TryFrom<ModuleListResponse> for ServiceDiscoveryState {
    type Error = ConduitSdkError;

    fn try_from(grpc: ModuleListResponse) -> Result<Self, Self::Error> {
        let module_states = grpc
            .modules
            .into_iter()
            .map(|state| state.try_into())
            .collect::<Result<Vec<ServiceDiscoveryModuleState>, _>>()?;
        let mut module_states: HashMap<ModuleName, ServiceDiscoveryModuleState> = module_states
            .into_iter()
            .try_fold(HashMap::new(), |mut acc, state| {
                let name: ModuleName = state.module_name.clone().into();
                if acc.contains_key(&name) {
                    return Err(ConduitSdkError::ApiBreakingChange(
                        "Invalid Config::ModuleResponse response message value. Duplicate modules[].moduleName!".to_string(),
                        None,
                    ));
                }
                acc.insert(name, state);
                Ok(acc)
            })?;
        let url_remap = ModuleUrlMappings::from_env()?;
        if let Some(mut url_remap) = url_remap {
            for (module_name, grpc_address) in url_remap.drain() {
                let module_state = module_states.remove(&module_name);
                if let Some(mut module_state) = module_state {
                    module_state.grpc_address = grpc_address;
                    module_states.insert(module_name, module_state);
                }
            }
        }
        Ok(ServiceDiscoveryState {
            module_states,
            last_updated: Utc::now(),
        })
    }
}

#[cfg(test)]
mod from_str_tests {
    use super::*;
    use crate::addr::{url_remap::MODULE_URL_REMAP_ENV, GrpcAddress, GrpcHost};
    use std::env;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_try_from() {
        let grpc_res = ModuleListResponse {
            modules: vec![
                ModuleResponse {
                    module_name: "authentication".to_string(),
                    url: "0.0.0.0:55160".to_string(),
                    serving: true,
                },
                ModuleResponse {
                    module_name: "database".to_string(),
                    url: "0.0.0.0:55161".to_string(),
                    serving: true,
                },
            ],
        };
        let expected_sd_state = ServiceDiscoveryState {
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
        let sd_state: ServiceDiscoveryState = grpc_res.try_into().unwrap();
        assert_eq!(sd_state, expected_sd_state);
    }

    #[test]
    fn test_try_from_module_url_remapped() {
        let json_mappings = "{\"authentication\" : \"0.0.0.0:55160\"}".to_string();
        unsafe {
            env::set_var(MODULE_URL_REMAP_ENV, json_mappings);
        }
        let grpc_res = ModuleListResponse {
            modules: vec![
                ModuleResponse {
                    module_name: "authentication".to_string(),
                    url: "conduit-authentication:55160".to_string(),
                    serving: true,
                },
                ModuleResponse {
                    module_name: "database".to_string(),
                    url: "0.0.0.0:55161".to_string(),
                    serving: true,
                },
            ],
        };
        let expected_sd_state = ServiceDiscoveryState {
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
        let sd_state: ServiceDiscoveryState = grpc_res.try_into().unwrap();
        unsafe {
            env::remove_var(MODULE_URL_REMAP_ENV);
        }
        assert_eq!(sd_state, expected_sd_state);
    }
}
