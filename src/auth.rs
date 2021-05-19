extern crate reqwest;

use serde_json::{Value};


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

pub async fn auth_to_api(client: reqwest::Client, api_key: &str){
    let curr_ts = sign::current_ts();

    let auth = client.get()
}