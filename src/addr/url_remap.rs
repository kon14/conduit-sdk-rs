use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::addr::GrpcAddress;
use crate::error::ConduitSdkError;
use crate::sd::ModuleName;

pub(crate) const MODULE_URL_REMAP_ENV: &str = "MODULE_URL_REMAP";

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct ModuleUrlMappings(pub(crate) HashMap<ModuleName, GrpcAddress>);

impl ModuleUrlMappings {
    pub(crate) fn from_env() -> Result<Option<Self>, ConduitSdkError> {
        let url_remap_str = match std::env::var(MODULE_URL_REMAP_ENV).ok() {
            Some(str) => str,
            None => return Ok(None),
        };
        let url_remaps: Result<HashMap<String, String>, serde_json::Error> =
            serde_json::from_str(&url_remap_str);
        let url_remaps = url_remaps
            .map_err(|_| ConduitSdkError::InvalidEnvValue(url_remap_str.clone()))?
            .into_iter()
            .map(|(name, url)| {
                let module_name: ModuleName = name
                    .parse()
                    .map_err(|_| ConduitSdkError::InvalidEnvValue(url_remap_str.clone()))?;
                let grpc_address: GrpcAddress = url
                    .parse()
                    .map_err(|_| ConduitSdkError::InvalidEnvValue(url_remap_str.clone()))?;
                Ok((module_name, grpc_address))
            })
            .collect::<Result<HashMap<ModuleName, GrpcAddress>, ConduitSdkError>>()?;
        Ok(Some(ModuleUrlMappings(url_remaps)))
    }
}

impl std::ops::Deref for ModuleUrlMappings {
    type Target = HashMap<ModuleName, GrpcAddress>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ModuleUrlMappings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod from_str_tests {
    use super::*;
    use crate::addr::GrpcHost;
    use std::env;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_from_env_valid() {
        let expected_mappings = ModuleUrlMappings(
            vec![(
                ModuleName::Authentication,
                GrpcAddress {
                    host: GrpcHost::Ip(IpAddr::from(Ipv4Addr::new(0, 0, 0, 0))),
                    port: 55160,
                },
            )]
            .into_iter()
            .collect(),
        );
        let json_mappings = "{\"authentication\" : \"0.0.0.0:55160\"}".to_string();
        unsafe {
            env::set_var(MODULE_URL_REMAP_ENV, json_mappings);
        }
        let env_mappings = ModuleUrlMappings::from_env().unwrap().unwrap();
        unsafe {
            env::remove_var(MODULE_URL_REMAP_ENV);
        }
    }
}
