extern crate reqwest;

use serde_json::{Value, json};
use reqwest::header;
use std::env;

#[path = "utils.rs"]
mod utils;

const API_ENDPOINT: &str = "https://ftx.com/api";


pub struct FtxApiClient{
    pub api_key: String,
    pub api_secret: String,
    pub request_client: reqwest::Client,
}

impl FtxApiClient{

    pub async fn fetch_historical_data(&self, market: &str, resolution: &str) -> Result<Value, Box<dyn std::error::Error>>{
        let data = self.request_client.get(format!("https://ftx.com/api/markets/{}/candles?resolution={}&limit=20", market, resolution))
            .send()
            .await?
            .json()
            .await?;    

        Ok(data)
    }

    fn auth_header(&self, endpoint: &str, method: &str) -> Result<header::HeaderMap, ()>{
        let signature: (String,String) = utils::signature(&self.api_secret, endpoint, method).unwrap();

        let mut headers = header::HeaderMap::new();
        headers.insert("FTX-KEY", self.api_key.parse().unwrap());
        headers.insert("FTX-SIGN", signature.0.parse().unwrap());
        headers.insert("FTX-TS", signature.1.parse().unwrap());

        Ok(headers)
    }

    pub async fn get_balance(&self) -> Result<Value, Box<dyn std::error::Error>>{
        let url = format!("{}{}", API_ENDPOINT, "/wallet/balances");
        let auth_header = self.auth_header("/wallet/balances", "GET").unwrap();
        let balance_request = self.request_client.get(url)
            .headers(auth_header)
            .send()
            .await?
            .json()
            .await?;

        Ok(balance_request)
    }
}
