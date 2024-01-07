use sea_orm::DatabaseConnection;

use crate::db::check_add_supermarket_info;

mod fetch;

pub async fn fetch(
    db: &mut DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let products = fetch::fetch_countdown_data().await?;
    let supermarket_id = check_add_supermarket_info(db, "Countdown Online", "Countdown", "Online", "online").await?;


    Ok(())
}
