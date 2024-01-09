use std::cmp;

use log::{warn, info};
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait};

use crate::db::{check_add_supermarket_info, entities::{product_db, supermarket_price}};

use self::product_matcher::get_price;

mod fetch;
mod product_matcher;

pub async fn fetch(
    db: &mut DatabaseConnection,
    products: &mut Vec<product_db::ActiveModel>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("COUNTDOWN STARTING");


    let supermarket_id = check_add_supermarket_info(db, "Countdown Online", "Countdown", "Online", "online").await?;

    let store_prices = fetch::fetch_countdown_data().await?;

    let product_ids = product_matcher::match_products(&store_prices, products, db).await?;

    info!("Uploading price data...");
    let matched_prices = store_prices.iter().zip(product_ids.iter()).map(|(x, y)| {

        if *y < 0 {
            // What????
            warn!("Product deemed invalid: {:?}, skipping", x.name);
            return None
        }

        Some(supermarket_price::ActiveModel {
            product_id: Set(y.clone()),
            supermarket_id: Set(supermarket_id.clone()),
            price: Set(get_price(x)),
            on_special: Set(Some(x.price.isSpecial)),
            original_price: Set(x.price.originalPrice),
            ..Default::default()
        })
    }).filter_map(|f| f).collect::<Vec<supermarket_price::ActiveModel>>();

    for price in matched_prices {
        price.save(db).await?;
    }

    info!("COUNTDOWN COMPLETE");

    Ok(())
}
