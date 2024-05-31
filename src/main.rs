use reqwest;

use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};

#[tokio::main]
async fn main -> Result<(), reqwest::Error> {
    let resp = reqwest::Client::new()
        .get("https://api.binance.com/sapi/v1/convert/exchangeInfo")
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", resp);
    
    Ok(())
}

//GET /api/v1/exchangeInfo