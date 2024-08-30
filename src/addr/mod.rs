pub(crate) mod url_remap;

use prost::bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use thiserror::Error;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum GrpcHost {
    Ip(IpAddr),
    Hostname(String),
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GrpcAddress {
    pub host: GrpcHost,
    pub port: u16,
}

#[derive(Error, Debug)]
pub enum GrpcAddressParseError {
    #[error("GrpcAddressParseError: Invalid format")]
    InvalidFormat,
    #[error("GrpcAddressParseError: Invalid host ({0})")]
    InvalidHost(String),
    #[error("GrpcAddressParseError: Invalid port ({0})")]
    InvalidPort(String),
}

impl std::str::FromStr for GrpcAddress {
    type Err = GrpcAddressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let (host_str, port) = match (parts.next(), parts.next(), parts.next()) {
            (Some(host), Some(port), None) => Ok((host, port)),
            _ => return Err(GrpcAddressParseError::InvalidFormat),
        }?;
        let ip = host_str
            .parse()
            .map(|ip: IpAddr| GrpcHost::Ip(ip))
            .map_err(|_| GrpcAddressParseError::InvalidHost(host_str.to_string()));
        let host = match ip {
            Ok(host) => host,
            Err(_) => parse_hostname(host_str)?,
        };
        let port: u16 = port
            .parse()
            .map_err(|_| GrpcAddressParseError::InvalidPort(port.into()))?;
        Ok(GrpcAddress { host, port })
    }
}

fn parse_hostname(str: &str) -> Result<GrpcHost, GrpcAddressParseError> {
    // TODO: additional validations
    let hostname = str.trim().to_string();
    if hostname.len() > 0 {
        Ok(GrpcHost::Hostname(hostname))
    } else {
        Err(GrpcAddressParseError::InvalidHost(str.to_string()))
    }
}

impl From<GrpcAddress> for Bytes {
    fn from(address: GrpcAddress) -> Self {
        let address = address.to_string(); // prefixed by gRPC protocol scheme
        Bytes::from(address.into_bytes())
    }
}

impl std::fmt::Display for GrpcHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrpcHost::Ip(ip) => write!(f, "{ip}"),
            GrpcHost::Hostname(hostname) => write!(f, "{hostname}"),
        }
    }
}

impl std::fmt::Display for GrpcAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "grpc://{}:{}", self.host, self.port)
    }
}

impl std::fmt::Debug for GrpcAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[cfg(test)]
mod from_str_tests {
    use super::*;

    #[test]
    fn test_parse_valid_address() {
        let address_str = "0.0.0.0:55152";
        let expected_address = GrpcAddress {
            host: GrpcHost::Ip(IpAddr::from(std::net::Ipv4Addr::new(0, 0, 0, 0))),
            port: 55152,
        };
        let parsed_address = address_str.parse::<GrpcAddress>().unwrap();
        assert_eq!(parsed_address, expected_address);
    }

    #[test]
    fn test_parse_invalid_hostname() {
        let address_str = ":55152";
        let result = address_str.parse::<GrpcAddress>();
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            GrpcAddressParseError::InvalidHost(_)
        ));
    }

    // #[test]
    // fn test_parse_invalid_ip() {
    //     let address_str = "invalid_ip:55152";
    //     let result = address_str.parse::<GrpcAddress>();
    //     assert!(result.is_err());
    //     assert!(matches!(
    //         result.err().unwrap(),
    //         GrpcAddressParseError::InvalidIp(_, _)
    //     ));
    // }

    #[test]
    fn test_parse_invalid_port() {
        let address_str = "0.0.0.0:not_a_port";
        let result = address_str.parse::<GrpcAddress>();
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            GrpcAddressParseError::InvalidPort(_)
        ));
    }

    #[test]
    fn test_parse_missing_port() {
        let address_str = "0.0.0.0";
        let result = address_str.parse::<GrpcAddress>();
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            GrpcAddressParseError::InvalidFormat
        ));
    }

    #[test]
    fn test_parse_extra_colon() {
        let address_str = "0.0.0.0:55152:";
        let result = address_str.parse::<GrpcAddress>();
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            GrpcAddressParseError::InvalidFormat
        ));
    }
}

#[cfg(test)]
mod to_string_tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let grpc_address = GrpcAddress {
            host: GrpcHost::Ip(IpAddr::from(std::net::Ipv4Addr::new(0, 0, 0, 0))),
            port: 55152,
        };
        let address_str = "grpc://0.0.0.0:55152";
        let grpc_address_str = grpc_address.to_string();
        assert_eq!(grpc_address_str, address_str);
    }

    #[test]
    fn test_into_bytes() {
        let grpc_address = GrpcAddress {
            host: GrpcHost::Ip(IpAddr::from(std::net::Ipv4Addr::new(0, 0, 0, 0))),
            port: 55152,
        };
        let grpc_address_bytes: Bytes = grpc_address.into();
        let expected_bytes: Bytes = "grpc://0.0.0.0:55152".into();
        assert_eq!(grpc_address_bytes, expected_bytes);
    }
}
