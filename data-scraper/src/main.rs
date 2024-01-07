use std::time::Duration;

use log::{error, info};

use crate::supermarkets::super_fetch;

mod config;
mod db;
mod supermarkets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_flexible_env_logger::try_init_with("INFO").unwrap();

    let mut db = db::connect().await;

    info!("Starting app");
    loop {
        super_fetch(&mut db).await?;
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
