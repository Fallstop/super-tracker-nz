use std::env;

use chrono::NaiveTime;
use log::info;
use once_cell::sync::Lazy;



pub static DATA_OUT_DIR: &str = "dataout_tmp";

pub struct EnvConfig {
    pub db_connection_uri: String,
    pub max_products_scrape: usize,
}

pub static CONFIG: Lazy<EnvConfig> = Lazy::new(|| {
    if dotenv::dotenv().is_err() {
        info!("No .env file found");
    }

    EnvConfig {
        db_connection_uri: env::var("DATABASE_URL")
            .expect("Missing DATABASE_URL environment variable"),
        max_products_scrape: env::var("MAX_PRODUCTS_SCRAPE")
            .unwrap_or(String::from("30000"))
            .parse::<usize>()
            .expect("MAX_PRODUCTS_SCRAPE must be a number"),
    }
});