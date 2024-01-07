use log::info;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::CONFIG;

pub async fn connect() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(&CONFIG.db_connection_uri);
    opt.sqlx_logging_level(log::LevelFilter::Debug);
    
    let db = Database::connect(opt).await.unwrap();

    info!("Connected to DB, running migrations...");
    migration::Migrator::up(&db, None).await.unwrap();
    info!("Migrations complete");

    db
}
