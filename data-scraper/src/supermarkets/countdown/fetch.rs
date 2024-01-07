use std::{time::Duration, fs};

use log::info;
use reqwest::header;
use serde::Deserialize;

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
    variety: String,
    brand: String,
    slug: String,
    sku: String,
    unit: String,
    price: ApiResponsePrice,
    images: ApiResponseImages,
    quantity: ApiResponseQuantity,
    stockLevel: usize,
    eachUnitQuantity: Option<String>,
    averageWeightPerUnit: f32,
    size: ApiResponseSize,
    departments: Vec<ApiResponseDepartment>,
    subsAllowed: bool,
    supportsBothEachAndKgPricing: bool,
    availabilityStatus: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponsePrice {
    originalPrice: f32,
    salePrice: f32,
    savePrice: f32,
    savePercentage: f32,
    canShowSavings: bool,
    hasBonusPoints: bool,
    isClubPrice: bool,
    isSpecial: bool,
    isNew: bool,
    canShowOriginalPrice: bool,
    discount: Option<String>,
    total: Option<String>,
    isTargetedOffer: bool,
    averagePricePerSingleUnit: f32,
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
    min: f32,
    max: f32,
    increment: f32,
    value: Option<String>,
    quantityInOrder: Option<String>,
    purchasingQuantityString: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseSize {
    cupPrice: f32,
    cupMeasure: String,
    packageType: String,
    volumeSize: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseDepartment {
    id: usize,
    name: String,
}

pub async fn fetch_countdown_data() -> Result<Vec<ApiResponseItem>, Box<dyn std::error::Error + Send + Sync>> {
    info!("Fetching Countdown data!");
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "www.countdown.co.nz".parse()?);
    headers.insert("accept", "application/json, text/plain, */*".parse()?);
    headers.insert("accept-language", "en-GB,en-US;q=0.9,en;q=0.8".parse()?);
    headers.insert("cache-control", "no-cache".parse()?);
    headers.insert("content-type", "application/json".parse()?);
    headers.insert("pragma", "no-cache".parse()?);
    headers.insert("referer", "https,//www.countdown.co.nz/shop/browse/fish-seafood/salmon/smoked-salmon".parse()?);
    headers.insert("sec-ch-ua", "\"Chromium\";v=\"118\", \"Google Chrome\";v=\"118\", \"Not=A?Brand\";v=\"99\"".parse()?);
    headers.insert("sec-ch-ua-mobile", "?0".parse()?);
    headers.insert("sec-ch-ua-platform", "\"Linux\"".parse()?);
    headers.insert("sec-fetch-dest", "empty".parse()?);
    headers.insert("sec-fetch-mode", "cors".parse()?);
    headers.insert("sec-fetch-site", "same-origin".parse()?);
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36".parse()?);
    headers.insert("x-requested-with", "OnlineShopping.WebApp".parse()?);
    headers.insert("x-ui-ver", "7.30.266".parse()?);

    let api_client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()?;

    let response = api_client.execute(
        api_client
            .get(PRODUCT_API_URL)
            .query(&[
                ("target", "browse"),
                ("inStockProductsOnly", "false"),
                ("page", "1"),
                ("size", "10")
            ])
            .build()?
    ).await?;

    let api_response = response.json::<ApiResponseRoot>().await?;

    // let response = fs::read_to_string("./countdown.json")?;
    // let api_response = serde_json::from_str::<ApiResponseRoot>(&response)?;



    let items = api_response.products.items;
    info!("Found {} items", items.len());

    return Ok(items)
}
