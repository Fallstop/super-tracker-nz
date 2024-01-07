use std::env;

use chrono::NaiveTime;
use log::info;
use once_cell::sync::Lazy;


pub static MAX_PRODUCT_SCRAPE: usize = 300;

pub struct EnvConfig {
    pub db_connection_uri: String,
}

pub static CONFIG: Lazy<EnvConfig> = Lazy::new(|| {
    if dotenv::dotenv().is_err() {
        info!("No .env file found");
    }

    EnvConfig {
        db_connection_uri: env::var("DATABASE_URL")
            .expect("Missing DATABASE_URL environment variable"),
    }
});