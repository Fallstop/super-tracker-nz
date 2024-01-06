use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

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
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_NotificationsSent_SongsPlayed")
                            .from(
                                NotificationsSent::NotificationsSent,
                                NotificationsSent::SongID,
                            )
                            .to(SongsPlayed::SongsPlayed, SongsPlayed::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(SupermarketPrice::SupermarketPrice).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SupermarketPrice {
    SupermarketPrice,
    Id,
    Timestamp,
    Supermarket,
    Price,
    OriginalPrice,
    OnSpecial
}
