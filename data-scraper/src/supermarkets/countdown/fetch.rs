use std::{time::Duration, io::{Error, ErrorKind}, cmp};

use fure::{backoff::{exponential, jitter}, policies::{cond, backoff}};
use log::info;
use rand::Rng;
use reqwest::header;
use serde::Deserialize;
use tokio::fs;

use crate::config::{MAX_PRODUCT_SCRAPE, self};

const PRODUCT_API_URL: &str = "https://www.countdown.co.nz/api/v1/products";

#[derive(Deserialize, Debug, Clone)]
struct ApiResponseRoot {
    pub products: ApiResponseItems,
    pub isSuccessful: bool,
}

#[derive(Deserialize, Debug, Clone)]
struct ApiResponseItems {
    pub items: Vec<ApiResponseItem>,
    pub totalItems: usize
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseItem {
    pub name: String,
    pub barcode: String,
    pub variety: Option<String>,
    pub brand: String,
    pub slug: String,
    pub sku: Option<String>,
    pub unit: String,
    pub price: ApiResponsePrice,
    pub images: ApiResponseImages,
    pub quantity: ApiResponseQuantity,
    pub stockLevel: usize,
    pub eachUnitQuantity: Option<String>,
    pub averageWeightPerUnit: Option<f32>,
    pub size: ApiResponseSize,
    pub departments: Vec<ApiResponseDepartment>,
    pub subsAllowed: bool,
    pub supportsBothEachAndKgPricing: bool,
    pub availabilityStatus: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponsePrice {
    pub originalPrice: Option<f32>,
    pub salePrice: Option<f32>,
    pub savePrice: Option<f32>,
    pub savePercentage: Option<f32>,
    pub canShowSavings: bool,
    pub hasBonusPoints: bool,
    pub isClubPrice: bool,
    pub isSpecial: bool,
    pub isNew: bool,
    pub canShowOriginalPrice: bool,
    pub discount: Option<String>,
    pub total: Option<String>,
    pub isTargetedOffer: bool,
    pub averagePricePerSingleUnit: Option<f32>,
    pub isBoostOffer: bool,
    pub purchasingUnitPrice: Option<String>,
    pub orderedPrice: Option<String>,
    pub isUsingOrderedPrice: bool,
    pub currentPricingMatchesOrderedPricing: Option<String>,
    pub extendedListPrice: Option<String>,
    pub originalAveragePricePerSingleUnit: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseImages {
    pub small: String,
    pub big: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseQuantity {
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub increment: Option<f32>,
    pub value: Option<String>,
    pub quantityInOrder: Option<String>,
    pub purchasingQuantityString: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseSize {
    pub cupPrice: Option<f32>,
    pub cupMeasure: Option<String>,
    pub packageType: Option<String>,
    pub volumeSize: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseDepartment {
    id: usize,
    name: String,
}

pub async fn fetch_countdown_data() -> Result<Vec<ApiResponseItem>, Box<dyn std::error::Error + Send + Sync>> {
    info!("Fetching Countdown data!");
    let mut rng = rand::thread_rng();

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
    let mut total_items = usize::MAX;
    let mut item_store = Vec::new();

    // Get all pages of data
    loop {
        let fetch_round_count = cmp::min(number_to_fetch, cmp::min(total_items,MAX_PRODUCT_SCRAPE) - item_store.len());

        info!("Loading data, page {}, {} items", page_num, fetch_round_count);


        let get_data = || async {
            let response = api_client.execute(
                api_client
                    .get(PRODUCT_API_URL)
                    .query(&[
                        ("target", "browse"),
                        ("inStockProductsOnly", "false"),
                        ("page", &page_num.to_string()),
                        ("size", &(fetch_round_count).to_string())
                    ])
                    .build().unwrap()
            ).await.map_err(|err| {
                std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
            })?;


            let contents = response.text().await.map_err(|err| {
                std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
            })?;


            let pretty_printed_json = jsonxf::pretty_print(&contents).unwrap_or(contents.to_owned());
            fs::write(
                format!("{}/countdown_{}.json", config::DATA_OUT_DIR, page_num),
                pretty_printed_json
            ).await?;

            match serde_json::from_str::<ApiResponseRoot>(&contents) {
                Ok(api_response) => {
                    Ok(api_response)
                },
                Err(e) => {
                    info!("Error parsing response from Countdown API: {}", e);
                    info!("Response: {}", contents);
                    Err(Box::new(Error::new(ErrorKind::Other, "Error parsing response from Countdown API")))
                }
            }
        };


        let exp_backoff = exponential(Duration::from_secs(1), 2, Some(Duration::from_secs(20)))
            .map(jitter);
        let policy = cond(backoff(exp_backoff), |result| !matches!(result, Some(Ok(_))));

        // Getting the data
        let api_response = fure::retry(get_data, policy).await?;
        
        let items = api_response.products.items;
        total_items = api_response.products.totalItems;
        info!("Found {} items, out of {}, (scrape max: {})", item_store.len(), api_response.products.totalItems,MAX_PRODUCT_SCRAPE);


        item_store.extend(items);



        if item_store.len() >= total_items || item_store.len() >= MAX_PRODUCT_SCRAPE {
            break;
        }
        page_num+=1;
        let gitter_ms = rng.gen_range(0..500);
        tokio::time::sleep(Duration::from_millis(1000 + gitter_ms)).await;
    }

    

    Ok(item_store)

}
