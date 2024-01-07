use super::entities::{prelude::*, supermarkets, product_db};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sea_orm::{ColumnTrait, Condition, EntityTrait};



pub async fn get_products(db: &mut DatabaseConnection) -> Result<Vec<product_db::Model>, Box<dyn std::error::Error + Send + Sync>> {
    let query = ProductDb::find()
        .all(db).await?;

    return Ok(query);
}

pub async fn update_products(db: &mut DatabaseConnection, products: Vec<product_db::Model>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // products.iter().map(|x| {
    //     x
    // })
    Ok(())
}