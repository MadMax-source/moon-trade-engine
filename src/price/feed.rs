use std::env;
use reqwest::header::{ACCEPT, HeaderMap, HeaderValue};
use serde_json::Value;
use dotenvy::dotenv;

use crate::constants::{JUPITER_PRICE_URL, WSOL_MINT};

pub async fn fetch_sol_price_usd() -> Result<f64, Box<dyn std::error::Error>> {
    let api_key = env::var("JUP_API_KEY").unwrap_or_default();

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    if !api_key.is_empty() {
        headers.insert("x-api-key", HeaderValue::from_str(&api_key)?);
    }

    let client = reqwest::Client::new();
    let res = client.get(JUPITER_PRICE_URL).headers(headers).send().await?;

    if !res.status().is_success() {
        return Err(format!("HTTP {}", res.status()).into());
    }

    let json: Value = res.json().await?;
    let sol = json.get(WSOL_MINT).ok_or("SOL price not available")?;
    let sol_price_usd = sol["usdPrice"].as_f64().ok_or("Invalid price")?;

    Ok(sol_price_usd)
}
