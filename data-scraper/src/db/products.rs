use super::entities::{prelude::*, supermarkets, product_db};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sea_orm::{ColumnTrait, Condition, EntityTrait};



pub async fn get_products(db: &mut DatabaseConnection) -> Result<Vec<product_db::ActiveModel>, Box<dyn std::error::Error + Send + Sync>> {
    let query = ProductDb::find()
        .all(db).await?;

    let active_models = query.into_iter().map(|x| x.into()).collect::<Vec<product_db::ActiveModel>>();

    return Ok(active_models);
}