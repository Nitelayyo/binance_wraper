use reqwest;

use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};

#[derive(Debug, Serialize, Deserialize)]
struct Pair{
    #[serde(rename = "fromAsset")]
    from_asset: String,
    #[serde(rename = "toAsset")]
    to_asset: String,
    #[serde(rename = "fromAssetMinAmount")]
    fom_asset_min_ammount: String,
    #[serde(rename = "fromAssetMaxAmount")]
    fom_asset_max_ammount: String,
    #[serde(rename = "toAssetMinAmount")]
    to_asset_min_ammount: String,
    #[serde(rename = "toAssetMaxAmount")]
    to_asset_max_ammount: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let resp: Vec<Pair> = reqwest::Client::new()
        .get("https://api.binance.com/sapi/v1/convert/exchangeInfo")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", resp);
    
    Ok(())
}

//GET /api/v1/exchangeInfo