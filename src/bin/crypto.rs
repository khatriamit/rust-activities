use dotenv::dotenv;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct USD {
    price: f64,
    last_updated: String,
}

#[derive(Debug, Deserialize)]
struct Quotes {
    USD: USD,
}

#[derive(Debug, Deserialize)]
struct Status {
    timestamp: String,
    error_code: i32,
    error_message: String,
    elapsed: i32,
    credit_count: i32,
    // notice: None
}

#[derive(Debug, Deserialize)]
struct Data {
    id: i32,
    symbol: String,
    name: String,
    amount: i32,
    last_updated: String,
    quote: Quotes,
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    // status: Status,
    data: Data,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("API_KEY")?;

    let mut headers = HeaderMap::new();

    headers.insert("X-CMC_PRO_API_KEY", api_key.parse().unwrap());

    let client = reqwest::Client::new();
    let resp = client
        .get("https://pro-api.coinmarketcap.com/v1/tools/price-conversion?id=1&amount=1")
        .headers(headers)
        .send()
        .await?;

    // println!("{:#?}", resp);
    let resp_json = resp.json::<ResponseData>().await?;
    println!("Response JSON {:#?}", resp_json);

    Ok(())
}
