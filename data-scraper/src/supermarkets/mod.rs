use sea_orm::DatabaseConnection;

pub mod countdown;


pub async fn super_fetch(db: &mut DatabaseConnection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    countdown::fetch(db).await?;

    Ok(())
}