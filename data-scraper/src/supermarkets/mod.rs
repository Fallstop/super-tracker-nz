use sea_orm::DatabaseConnection;

use crate::db::get_products;

pub mod countdown;


pub async fn super_fetch(db: &mut DatabaseConnection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut products = get_products(db).await?;

    countdown::fetch(db,&mut products).await?;

    Ok(())
}