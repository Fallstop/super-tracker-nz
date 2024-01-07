use std::{time::Duration, fs, io::{Error, ErrorKind}};

use log::info;
use reqwest::header;
use serde::Deserialize;

use crate::config::MAX_PRODUCT_SCRAPE;

const PRODUCT_API_URL: &str = "https://www.countdown.co.nz/api/v1/products";

#[derive(Deserialize, Debug, Clone)]
struct ApiResponseRoot {
    products: ApiResponseItems,
    isSuccessful: bool,
}

#[derive(Deserialize, Debug, Clone)]
struct ApiResponseItems {
    items: Vec<ApiResponseItem>,
    totalItems: usize
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseItem {
    name: String,
    barcode: String,
    variety: Option<String>,
    brand: String,
    slug: String,
    sku: Option<String>,
    unit: String,
    price: ApiResponsePrice,
    images: ApiResponseImages,
    quantity: ApiResponseQuantity,
    stockLevel: usize,
    eachUnitQuantity: Option<String>,
    averageWeightPerUnit: Option<f32>,
    size: ApiResponseSize,
    departments: Vec<ApiResponseDepartment>,
    subsAllowed: bool,
    supportsBothEachAndKgPricing: bool,
    availabilityStatus: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponsePrice {
    originalPrice: Option<f32>,
    salePrice: Option<f32>,
    savePrice: Option<f32>,
    savePercentage: Option<f32>,
    canShowSavings: bool,
    hasBonusPoints: bool,
    isClubPrice: bool,
    isSpecial: bool,
    isNew: bool,
    canShowOriginalPrice: bool,
    discount: Option<String>,
    total: Option<String>,
    isTargetedOffer: bool,
    averagePricePerSingleUnit: Option<f32>,
    isBoostOffer: bool,
    purchasingUnitPrice: Option<String>,
    orderedPrice: Option<String>,
    isUsingOrderedPrice: bool,
    currentPricingMatchesOrderedPricing: Option<String>,
    extendedListPrice: Option<String>,
    originalAveragePricePerSingleUnit: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseImages {
    small: String,
    big: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseQuantity {
    min: Option<f32>,
    max: Option<f32>,
    increment: Option<f32>,
    value: Option<String>,
    quantityInOrder: Option<String>,
    purchasingQuantityString: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseSize {
    cupPrice: Option<f32>,
    cupMeasure: Option<String>,
    packageType: Option<String>,
    volumeSize: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseDepartment {
    id: usize,
    name: String,
}

pub async fn fetch_countdown_data() -> Result<Vec<ApiResponseItem>, Box<dyn std::error::Error + Send + Sync>> {
    info!("Fetching Countdown data!");
    let number_to_fetch = 120;
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "www.countdown.co.nz".parse()?);
    headers.insert("accept", "application/json, text/plain, */*".parse()?);
    headers.insert("accept-language", "en-GB,en-US;q=0.9,en;q=0.8".parse()?);
    headers.insert("cache-control", "no-cache".parse()?);
    headers.insert("User-Agent", "Yes/1.0.0".parse()?);
    headers.insert("x-requested-with", "OnlineShopping.WebApp".parse()?);
    headers.insert("x-ui-ver", "7.30.266".parse()?);

    let api_client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .connect_timeout(Duration::from_secs(5))
        .connection_verbose(true)
        .timeout(Duration::from_secs(10))
        .build()?;

    let mut page_num = 1;
    let mut total_items = 0;
    let mut item_store = Vec::new();
    loop {
        let response = api_client.execute(
            api_client
                .get(PRODUCT_API_URL)
                .query(&[
                    ("target", "browse"),
                    ("inStockProductsOnly", "false"),
                    ("page", &page_num.to_string()),
                    ("size", &number_to_fetch.to_string())
                ])
                .build()?
        ).await?;
        info!("Got response from Countdown API");

        let contents = response.text().await?;
        (match serde_json::from_str::<ApiResponseRoot>(&contents) {
            Ok(api_response) => {
                let items = api_response.products.items;
                item_store.extend(items);
                total_items = api_response.products.totalItems;
                info!("Found {} items, out of {}, (scrape max: {})", item_store.len(), api_response.products.totalItems,MAX_PRODUCT_SCRAPE);
                Ok(())
            },
            Err(e) => {
                info!("Error parsing response from Countdown API: {}", e);
                info!("Response: {}", contents);
                Err(Box::new(Error::new(ErrorKind::Other, "Error parsing response from Countdown API")))
            }
        })?;
        if item_store.len() >= total_items || item_store.len() >= MAX_PRODUCT_SCRAPE {
            break;
        }
        page_num+=1;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    

    Ok(item_store)

}
