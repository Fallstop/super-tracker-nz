use std::time::Duration;

use log::{error, info};
use tokio::fs;

use crate::{supermarkets::super_fetch, config::DATA_OUT_DIR};

mod config;
mod db;
mod supermarkets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_flexible_env_logger::try_init_with("INFO").unwrap();

    fs::create_dir_all(DATA_OUT_DIR).await?;

    let mut db = db::connect().await;

    info!("Starting app");
    loop {
        super_fetch(&mut db).await?;
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
