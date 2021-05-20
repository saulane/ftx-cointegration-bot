extern crate reqwest;

use serde_json::{Value, json};
use reqwest::header;

#[path = "sign.rs"]
mod sign;

const API_ENDPOINT: &str = "https://ftx.com/api";

async fn fetch_historical_data(client: reqwest::Client, market: &str, resolution: &str) -> Result<String, Box<dyn std::error::Error>>{
    let data = client.get(format!("https://ftx.com/api/markets/{}/candles?resolution={}&limit=20", market, resolution))
        .send()
        .await?
        .text()
        .await?;    

    Ok(data)
}

pub async fn get_jsoned_data(client: reqwest::Client, market: &str, resolution: &str) -> serde_json::Result<Value>{
    let data = fetch_historical_data(client, market, resolution).await.unwrap();
    let data_json: Value = serde_json::from_str(&data).expect("Probleme parsing to JSON");

    Ok(data_json)
}

 fn auth_header(api_key: &str, api_secret: &str, endpoint: &str, method: &str) -> Result<header::HeaderMap, ()>{
    let curr_ts:u128 = sign::current_ts();

    let signature: String = sign::signature(api_secret, curr_ts, endpoint, method).unwrap();

    let mut headers = header::HeaderMap::new();
    headers.insert("FTX-KEY", api_key.parse().unwrap());
    headers.insert("FTX-SIGN", signature.parse().unwrap());
    headers.insert("FTX-TS", curr_ts.to_string().parse().unwrap());

    println!("{:?}", headers);

    Ok(headers)
}

pub async fn get_balance(client: reqwest::Client, api_key: &str, api_secret: &str) -> Result<Value, Box<dyn std::error::Error>>{
    let url = format!("{}{}", API_ENDPOINT, "/wallet/balances");
    println!("{}", url);

    let header = auth_header(api_key,api_secret, "/wallet/balances", "GET").unwrap();
    println!("{:?}",header);

    let balance_request = client.get(url)
        .headers(header)
        .send()
        .await?
        .json()
        .await?;

    Ok(balance_request)
}