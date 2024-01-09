use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Supermarkets::Supermarkets)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Supermarkets::SupermarketId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Supermarkets::Name)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Supermarkets::BrandName)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Supermarkets::Location)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Supermarkets::LocationID)
                            .string()
                            .not_null()
                    )
                    .to_owned()
            ).await?;
        
        manager
            .create_table(
                Table::create()
                    .table(ProductDB::ProductDB)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductDB::ProductID)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                        )
                    .col(
                        ColumnDef::new(ProductDB::ProductTitle)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(ProductDB::ProductVariety)
                            .string()
                    )
                    .col(
                        ColumnDef::new(ProductDB::ProductBrand)
                            .string()
                    )
                    .col(
                        ColumnDef::new(ProductDB::ImageURL)
                            .string()
                    )
                    .col(
                        ColumnDef::new(ProductDB::Barcode)
                            .string()
                    )
                    .col(
                        ColumnDef::new(ProductDB::Size)
                            .float()
                    )
                    .col(
                        ColumnDef::new(ProductDB::Unit)
                            .string()
                    )
                    .col(
                        ColumnDef::new(ProductDB::Quantity)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(ProductDB::FirstIndexTimestamp)
                            .date_time()
                            .default(Expr::current_timestamp())
                            .not_null()
                    )
                    .to_owned()
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(SupermarketPrice::SupermarketPrice)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SupermarketPrice::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SupermarketPrice::Timestamp)
                            .date_time()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SupermarketPrice::SupermarketID)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_SupermarketPrice_SupermarketId")
                            .from(
                                SupermarketPrice::SupermarketPrice,
                                SupermarketPrice::SupermarketID,
                            )
                            .to(Supermarkets::Supermarkets, Supermarkets::SupermarketId),
                    )
                    .col(
                        ColumnDef::new(SupermarketPrice::ProductID)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_SupermarketPrice_ProductId")
                            .from(SupermarketPrice::SupermarketPrice, SupermarketPrice::ProductID)
                            .to(ProductDB::ProductDB, ProductDB::ProductID),
                    )
                    .col(
                        ColumnDef::new(SupermarketPrice::Price)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SupermarketPrice::OnSpecial)
                            .boolean()
                            .default(false)
                    )
                    .col(
                        ColumnDef::new(SupermarketPrice::OriginalPrice)
                            .float()
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SupermarketPrice::SupermarketPrice).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Supermarkets::Supermarkets).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ProductDB::ProductDB).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum SupermarketPrice {
    SupermarketPrice,
    Id,
    Timestamp,
    SupermarketID,
    ProductID,
    Price,
    OriginalPrice,
    OnSpecial
}

#[derive(DeriveIden)]
enum Supermarkets {
    Supermarkets,
    SupermarketId,
    Name,
    BrandName,
    Location,
    LocationID
}

#[derive(DeriveIden)]
enum ProductDB {
    ProductDB,
    ProductTitle,
    ProductVariety,
    ProductBrand,
    Barcode,
    ProductID,
    Size,
    Unit,
    Quantity,
    ImageURL,
    FirstIndexTimestamp
}