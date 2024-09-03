use chrono::Utc;
use conduit_sdk_rs::addr::GrpcAddress;
use conduit_sdk_rs::connect;
use std::time::Duration;
use tokio::time::sleep;

const CONDUIT_SERVER_URL_ENV: &str = "CONDUIT_SERVER_URL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let conduit_url= std::env::var(CONDUIT_SERVER_URL_ENV)
        .unwrap_or_else(|_| panic!("Unable to detect Conduit server's URL! Please set {CONDUIT_SERVER_URL_ENV}. (Example: 0.0.0.0:55152)"));

    let grpc_address: GrpcAddress = conduit_url.parse().unwrap();

    let conduit = connect(grpc_address, Some(10), None).await.unwrap();

    let sd_state = conduit.monitor_modules().await?;

    loop {
        println!("\nService Discovery State @ {}", Utc::now());
        println!("{sd_state:#?}");
        sleep(Duration::from_millis(500)).await;
    }
}
