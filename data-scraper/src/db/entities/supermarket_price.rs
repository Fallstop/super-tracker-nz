//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "supermarket_price")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub timestamp: DateTime,
    pub supermarket_id: i32,
    pub product_id: i32,
    #[sea_orm(column_type = "Float")]
    pub price: f32,
    pub on_special: Option<bool>,
    #[sea_orm(column_type = "Float", nullable)]
    pub original_price: Option<f32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::product_db::Entity",
        from = "Column::ProductId",
        to = "super::product_db::Column::ProductId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ProductDb,
    #[sea_orm(
        belongs_to = "super::supermarkets::Entity",
        from = "Column::SupermarketId",
        to = "super::supermarkets::Column::SupermarketId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Supermarkets,
}

impl Related<super::product_db::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductDb.def()
    }
}

impl Related<super::supermarkets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Supermarkets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
