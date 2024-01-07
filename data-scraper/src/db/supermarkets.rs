use super::entities::{prelude::*, supermarkets};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};



pub async fn check_add_supermarket_info(db: &mut DatabaseConnection, name: &str, brand: &str, location: &str, location_id: &str)-> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
    let query = Supermarkets::find()
        .filter(
            Condition::all()
                .add(supermarkets::Column::LocationId.contains(location_id))
                .add(supermarkets::Column::BrandName.contains(brand))
        )
        .one(db).await?;

    if let Some(found_record) = query {
        return Ok(found_record.supermarket_id);
    } else {
        let new_supermarket = supermarkets::ActiveModel {
            brand_name: Set(brand.to_owned()),
            location: Set(location.to_owned()),
            location_id: Set(location_id.to_owned()),
            name: Set(name.to_owned()),
            ..Default::default()
        };
        return Ok(new_supermarket.insert(db).await?.supermarket_id);
    }
}