use std::{time::Duration, io::{Error, ErrorKind}, cmp};

use fure::{backoff::{exponential, jitter}, policies::{cond, backoff}};
use log::info;
use rand::Rng;
use regex::Regex;
use reqwest::{header, Client};
use tokio::fs;

use crate::{config, supermarkets::countdown::api_response::{self, ApiResponseItem}};

use super::api_response::{ApiProduct, ApiResponseRoot};

const PRODUCT_API_URL: &str = "https://www.countdown.co.nz/api/v1/products";



pub async fn fetch_countdown_data() -> Result<Vec<ApiProduct>, Box<dyn std::error::Error + Send + Sync>> {
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

    let mut item_store = Vec::new();

    for department in list_departments(&api_client).await? {
        let department_items = fetch_department(&department, api_client.clone()).await?;
        item_store.extend(department_items);
    }


    return Ok(item_store);
}



async fn fetch_department(department: &str, api_client: Client) -> Result<Vec<ApiProduct>, Box<dyn std::error::Error + Send + Sync>> {
    info!("[{}] Fetching Countdown data!", department);
    let mut rng = rand::thread_rng();

    let number_to_fetch = 120;
    

    let mut page_num = 1;
    let mut total_items = usize::MAX;
    let mut item_store = Vec::new();

    // Get all pages of data
    loop {
        let fetch_round_count = cmp::min(number_to_fetch, cmp::min(total_items,config::CONFIG.max_products_scrape) - item_store.len());

        info!("[{}] Loading data, page {}, {} items", department, page_num, fetch_round_count);

        let api_response = send_request(&api_client, Some(department), page_num, fetch_round_count).await?;
        
        let items = api_response.products.items;
        total_items = api_response.products.totalItems;


        item_store.extend(
            items
                .into_iter()
                .filter_map(|e| match e {
                    ApiResponseItem::Product(group) => Some(group),
                    _ => None,
                })
        );
        info!("[{}] Found {} items, out of {}, (scrape max: {})", department, item_store.len(), api_response.products.totalItems,config::CONFIG.max_products_scrape);



        if item_store.len() >= total_items || item_store.len() >= config::CONFIG.max_products_scrape {
            break;
        }
        page_num+=1;
        let gitter_ms = rng.gen_range(0..500);
        tokio::time::sleep(Duration::from_millis(1000 + gitter_ms)).await;
    }

    

    Ok(item_store)

}

async fn list_departments(api_client: &Client) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let api_response = send_request(api_client, None, 1, 1).await?;

    let human_department_names: Vec<String> = api_response.dasFacets.iter().map(|x| {
        x.name.clone()
    }).collect();

    let filter_names: Vec<String> = human_department_names.into_iter().map(|x| {
        let regex = Regex::new(r"(?m)\s+").unwrap();
        let cleaned = x.trim().chars().filter(|c| c.is_alphabetic() || c.is_whitespace()).collect::<String>().to_lowercase();
        regex.replace_all(&cleaned, "-").to_string()
    }).collect();

    Ok(filter_names)
}


async fn send_request(api_client: &Client, department: Option<&str>, page: usize, size: usize) -> Result<ApiResponseRoot, Box<dyn std::error::Error + Send + Sync>> {
    let get_data = || async {
        let page_num = page.to_string();
        let page_size = (size).to_string();

        let mut query_params = vec![
            ("target", "browse"),
            ("inStockProductsOnly", "false"),
            ("page", &page_num),
            ("size", &page_size),
        ];

        let filter: String;
        if let Some(department) = department {
            filter = format!("Department;;{department};false");
            query_params.push(("dasFilter", &filter));
        }

        let response = api_client.execute(
            api_client
                .get(PRODUCT_API_URL)
                .query(&query_params)
                .build().unwrap()
        ).await.map_err(|err| {
            Error::new(ErrorKind::Other, err.to_string())
        })?;


        let contents = response.text().await.map_err(|err| {
            Error::new(ErrorKind::Other, err.to_string())
        })?;


        let pretty_printed_json = jsonxf::pretty_print(&contents).unwrap_or(contents.to_owned());
        fs::write(
            format!("{}/countdown_{}_{}.json", config::DATA_OUT_DIR, department.unwrap_or("root"), page),
            pretty_printed_json.clone()
        ).await?;

        match serde_json::from_str::<ApiResponseRoot>(&pretty_printed_json) {
            Ok(api_response) => {
                Ok(api_response)
            },
            Err(e) => {
                info!("Error parsing response from Countdown API: {}", e);
                if contents.len() < 1000 {
                    info!("Response: {}", contents);
                }
                Err(Box::new(Error::new(ErrorKind::Other, "Error parsing response from Countdown API")))
            }
        }
    };


    let exp_backoff = exponential(Duration::from_secs(1), 2, Some(Duration::from_secs(20)))
        .map(jitter);
    let policy = cond(backoff(exp_backoff), |result| !matches!(result, Some(Ok(_))));

    // Getting the data
    Ok(fure::retry(get_data, policy).await?)
}