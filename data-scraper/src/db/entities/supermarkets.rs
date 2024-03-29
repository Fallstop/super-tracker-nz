//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "supermarkets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub supermarket_id: i32,
    pub name: String,
    pub brand_name: String,
    pub location: String,
    pub location_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::supermarket_price::Entity")]
    SupermarketPrice,
}

impl Related<super::supermarket_price::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SupermarketPrice.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
