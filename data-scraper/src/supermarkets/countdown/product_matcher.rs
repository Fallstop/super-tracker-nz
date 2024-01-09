use std::{collections::HashMap, cmp};

use log::info;
use sea_orm::{Set,NotSet, ActiveModelTrait};
use tokio::time::Instant;
use url::Url;

use crate::db::entities::product_db;

use super::api_response::ApiProduct;

pub async fn match_products(
    store_products: &Vec<ApiProduct>,
    db_products: &mut Vec<product_db::ActiveModel>,
    db: &mut sea_orm::DatabaseConnection,
) -> Result<Vec<i32>, Box<dyn std::error::Error + Send + Sync>> {

    info!("Matching Products! {}/{}", store_products.len(), db_products.len(),);
    let start_time = Instant::now();

    let mut novel_products: usize = 0;

    let mut matched_product_ids = Vec::new();

    for store_product in store_products.clone() {

        // Check valid price
        let store_price = get_price(&store_product);
        if store_price == 0.0 {
            info!("Price is 0.0 for product: {:?}, skipping", store_product.name);
            matched_product_ids.push(-1);
            continue;
        }


        // Check for a perfect ID match
        let matched_product = db_products.iter_mut().find(|db_product| {
            db_product.barcode.clone().unwrap().is_some_and(|x| x == store_product.barcode)
        });
        if let Some(matched_product) = matched_product {
            matched_product_ids.push(matched_product.product_id.clone().unwrap());
            continue;
        }

        // TODO: Imperfect ID match



        // Backup, create the product
        let (size, quantity, unit) = parse_size_unit(&store_product, store_price);
        
        let new_product = product_db::ActiveModel {
            product_title: Set(store_product.name),
            product_brand: Set(Some(store_product.brand)),
            barcode: Set(Some(store_product.barcode)),
            image_url: Set(Some(get_large_image(&store_product.images.big))),
            product_variety: Set(store_product.variety),
            quantity: Set(quantity),
            size: Set(size),
            unit: Set(unit),
            ..Default::default()
        };
        novel_products += 1;
        let db_entry = new_product.save(db).await?;

        // info!("Created new product: {:?}", db_entry);
        matched_product_ids.push(db_entry.product_id.clone().unwrap());
        db_products.push(db_entry);
    }

    info!("Matched {} products, in {}s!", matched_product_ids.len(), start_time.elapsed().as_millis() as f64 / 1000.0);
    info!("Created {} new products!", novel_products);

    Ok(matched_product_ids)
}

fn get_large_image(src: &str) -> String {
    let mut url = Url::parse(src).unwrap();
    
    // Remove all queries except impolicy
    let mut query_pairs: HashMap<_, _> = url.query_pairs().into_owned().collect();
    query_pairs.retain(|key, _| key == "impolicy");
    let new_query: String = query_pairs.into_iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&");
    url.set_query(Some(&new_query));

    return url.into()
}



fn parse_size_unit(store_product: &ApiProduct, store_price: f32) -> (Option<f32>, i32, Option<String>) {
    let mut size= None;
    let mut unit = None;
    let mut quantity = 1;

    // Size can sometimes be find in the "volumeSize" property,
    // or in the variant name
    // but sometimes it can only be found in the (price / cupPrice) * cupMeasure
    // property. So we need to check all.
    if let Some(volume_size) = &store_product.size.volumeSize {
        let (parsed_size, parsed_unit) = parse_unit(&volume_size);

        // Sometimes volume_size is actually "3pack", rather than an volume size
        if parsed_unit == "pack" {
            quantity = parsed_size as i32;
        } else {
            if !parsed_unit.is_empty() {
                unit = Some(parsed_unit);
            }
            size = Some(parsed_size);    
        }
    }

    // Check for unit in variant name
    if unit.is_none() {
        if let Some(variant_name) = &store_product.variety {
            let (parsed_size, parsed_unit) = parse_unit(&variant_name);

            if !parsed_unit.is_empty() && (parsed_size != 1.0) && !variant_name.contains(" ") {
                unit = Some(parsed_unit);
                size = Some(parsed_size);
            }
        }
    }
    // Overwrite if we find a cupMeasure and we didn't find a unit initially
    if unit.is_none() {
        if let (Some(cup_measure), Some(cup_price)) = (&store_product.size.cupMeasure, &store_product.size.cupPrice) {
            let (parsed_size, parsed_unit) = parse_unit(&cup_measure);

            let real_size = (parsed_size * (store_price / cup_price)) / quantity as f32;

            if parsed_unit == "ea" {
                // If we already have a quantity, then we don't need to set it again
                if quantity == 1 {
                    quantity = real_size.round() as i32;
                }
            } else {
                if !parsed_unit.is_empty() {
                    unit = Some(parsed_unit);
                }
                size = Some(real_size);
    
            }
    
    
        }    
    }

    // Quantity can be found in the "volumeSize" property

    (size, quantity, unit)
}

// EG: ("300kg" -> 300.0, "kg"), ("36g" -> 36.0, "g"), ("per kg" -> 1.0, "kg")
fn parse_unit(str: &str) -> (f32, String) {
    let parsed_unit = str
            .chars().rev()
            .take_while(|c| c.is_alphabetic() && *c!=' ')
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
    let size = str.replace(&parsed_unit, "").trim().to_string().parse::<f32>().unwrap_or(1.0);

    (size, parsed_unit)
}

pub fn get_price(store_product: &ApiProduct) -> f32 {
    store_product.price.salePrice.unwrap_or(store_product.price.originalPrice.unwrap_or(0.0))
}