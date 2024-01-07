mod fetch;

pub async fn fetch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    fetch::fetch_countdown_data().await?;

    Ok(())
}