use reqwest;

use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};

#[tokio::main] // дозволяє асинхронність
async fn main() {
    let resp = match reqwest::get("https://api.binance.com/sapi/v1/convert/exchangeInfo").await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => panic!("Error: {}", err)
    };
    println!("{}", resp)
}

//GET /api/v1/exchangeInfo