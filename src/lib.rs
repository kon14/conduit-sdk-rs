pub mod addr;
pub mod clients;
pub mod error;
pub mod modules;
pub mod prelude;
pub mod registry;
pub mod sd;

use crate::addr::GrpcAddress;
use crate::clients::health::{HealthClient, ServingStatus};
use crate::error::ConduitSdkError;
use crate::modules::conduit::Conduit;
use std::time::Duration;
use tonic::transport::Channel;

/// Connects to Conduit Core, returning an initialized `Conduit` instance.
///
/// A maximum amount of connection attempts may be specified (default: `None`), together with a backoff delay (default: `250ms`).
pub async fn connect(
    core_address: GrpcAddress,
    max_attempts: Option<u64>,
    backoff_delay: Option<Duration>,
) -> Result<Conduit, ConduitSdkError> {
    const CONDUIT_HEALTH_SERVICE: &str = "";
    let backoff_delay = backoff_delay.unwrap_or(Duration::from_millis(250));

    async fn attempt_conn(
        core_address: GrpcAddress,
        channel: Option<Channel>,
    ) -> Result<(ServingStatus, Channel), ConduitSdkError> {
        let channel = match channel {
            Some(channel) => channel,
            None => {
                let endpoint = Channel::from_shared(core_address).map_err(|err| {
                    ConduitSdkError::ConnectionFailure(
                        "Failed to establish a connection to Conduit Core. Invalid address provided!"
                            .to_string(),
                        Some(Box::new(err)),
                    )
                })?;
                let channel = endpoint.connect().await.map_err(|err| {
                    ConduitSdkError::ConnectionFailure(
                        "Failed to establish a connection to Conduit Core!".to_string(),
                        Some(Box::new(err)),
                    )
                })?;
                channel
            }
        };
        let health_client = HealthClient::new(channel.clone());
        let status = health_client
            .check(CONDUIT_HEALTH_SERVICE.to_string())
            .await?
            .status;
        Ok((status, channel))
    }

    println!("Waiting for Conduit...");
    let mut attempts = 0_u64;
    let mut channel: Option<Channel> = None;
    let channel = loop {
        attempts += 1;
        if let Some(max_attempts) = max_attempts {
            if attempts > max_attempts {
                return Err(ConduitSdkError::ConnectionFailure(
                    "Failed to establish a connection to Conduit Core. Timed out!".to_string(),
                    None,
                ));
            }
        }
        let (status, _channel) = attempt_conn(core_address.clone(), channel).await?;
        if status == ServingStatus::Serving {
            break _channel;
        }
        channel = Some(_channel);
        tokio::time::sleep(backoff_delay).await;
    };
    println!("Conduit connection established!");
    Ok(Conduit::new(core_address, channel))
}
