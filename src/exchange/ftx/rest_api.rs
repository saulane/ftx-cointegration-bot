extern crate reqwest;

#[path = "utils.rs"]
mod utils;
mod objects;

use serde_json::{Value, json};
use reqwest::header;
use std::{collections::HashMap, fmt::format};

use objects::{Balance, Market};


const API_ENDPOINT: &str = "https://ftx.com/api";



pub struct FtxApiClient{
    pub api_key: String,
    pub api_secret: String,
    pub request_client: reqwest::Client,
}

impl FtxApiClient{

    pub async fn get_market(&self, market: &str) -> Result<Market, Box<dyn std::error::Error>>{
        let data: Value = self.request_client.get(format!("https://ftx.com/api/markets/{}", market))
            .send()
            .await?
            .json()
            .await?;

        let res = data["result"].clone();

        let market: Market = serde_json::from_value(res)?;

        Ok(market)
    }

    pub async fn fetch_historical_data(&self, market: &str, resolution: &str) -> Result<Value, Box<dyn std::error::Error>>{
        let data = self.request_client.get(format!("https://ftx.com/api/markets/{}/candles?resolution={}&limit=20", market, resolution))
            .send()
            .await?
            .json()
            .await?;

        Ok(data)
    }

    fn auth_header(&self, endpoint: &str, method: &str) -> Result<header::HeaderMap, ()>{
        let signature: (String,String) = utils::signature(&self.api_secret, endpoint, method, "rest").unwrap();

        let mut headers = header::HeaderMap::new();
        headers.insert("FTX-KEY", self.api_key.parse().unwrap());
        headers.insert("FTX-SIGN", signature.0.parse().unwrap());
        headers.insert("FTX-TS", signature.1.parse().unwrap());

        Ok(headers)
    }

    fn url(&self, endpoint: &str) -> String{
        format!("{}{}",API_ENDPOINT, endpoint)
    }

    pub async fn get_balance(&self) -> Result<Balance, Box<dyn std::error::Error>>{
        let url = format!("{}{}", API_ENDPOINT, "/wallet/balances");
        let auth_header = self.auth_header("/wallet/balances", "GET").unwrap();
        let balance_request = self.request_client.get(url)
            .headers(auth_header)
            .send()
            .await?
            .json()
            .await?;

        let balance: Balance = serde_json::from_value(balance_request)?;

        Ok(balance)
    }

    pub async fn get_open_orders(&self) -> Result<Value, reqwest::Error>{
        let url = self.url("/orders?market=BTC-PERP");
        let auth_header = self.auth_header("/orders?market=BTC-PERP", "GET").unwrap();
        let open_orders = self.request_client.get(url)
            .headers(auth_header)
            .send()
            .await?
            .json()
            .await?;

        Ok(open_orders)
    }

    pub async fn get_orders_history(&self) -> Result<Value, reqwest::Error>{
        let url = self.url("/orders/history?market=BTC-PERP");
        let auth_header = self.auth_header("/orders/history?market=BTC-PERP", "GET").unwrap();
        let orders_history = self.request_client.get(url)
            .headers(auth_header)
            .send()
            .await?
            .json()
            .await?;

        Ok(orders_history)
    }

    pub async fn post_order(&self,market:&str, price: f64, size: f64, r#type:&str, reduceOnly: bool) -> Result<Value, Box<dyn std::error::Error>>{
        let orders_url = self.url("/orders");

        let side = if size>=0.0 {"buy"} else {"sell"};

        let price = if r#type == "limit" { price } else { 0.0 };
        let params_json = json!({
            "market": market,
            "size": size.abs(),
            "side": side,
            "type": r#type,
            "price": price,
            "reduceOnly": reduceOnly,
        });

        println!("{:?}",params_json);

        let url_params = format!("/orders{}",params_json.to_string());
        let auth_header = self.auth_header(&url_params, "POST").unwrap();

        let order = self.request_client.post(orders_url)
            .headers(auth_header)
            .json(&params_json)
            .send()
            .await?
            .json()
            .await?;

        println!("{:?}",order);

        Ok(order)
    }

    pub async fn modify_order(&self, order_id:&str, size: Option<f64>, price: Option<f64>) -> Result<Value, reqwest::Error>{
        let order_endpoint = format!("/orders/{}/modify", order_id);
        let url = self.url(&order_endpoint);

        let mut params = HashMap::new();

        match size{
            Some(i) => params.insert("size", i),
            None => None
        };

        match price{
            Some(i) => params.insert("price", i),
            None => None
        };

        let params_json:Value = serde_json::from_str(&format!("{:?}", params)).unwrap();
        println!("Json: {}", params_json);

        let url_params = format!("/orders/{}/modify{}",order_id,params_json.to_string());
        let auth_header = self.auth_header(&url_params, "POST").unwrap();
        
        let modify_order = self.request_client.post(url)
            .headers(auth_header)
            .json(&params_json)
            .send()
            .await?
            .json()
            .await?;

        Ok(modify_order)
    }

    pub async fn cancel_all_orders(&self) -> Result<Value, reqwest::Error>{
        let url = self.url("/orders");
        let auth_header = self.auth_header("/orders", "DELETE").unwrap(); 

        let cancel_orders = self.request_client.delete(url)
            .headers(auth_header)
            .send()
            .await?
            .json()
            .await?;

        Ok(cancel_orders)
    }

    pub async fn get_order_status(&self, order_id: &str) -> Result<Value, reqwest::Error>{
        let order_endpoint = format!("/orders/{}", order_id);
        let url = self.url(&order_endpoint);
        let auth_header = self.auth_header(&order_endpoint, "GET").unwrap(); 

        let order_status = self.request_client.get(url)
            .headers(auth_header)
            .send()
            .await?
            .json()
            .await?;

        Ok(order_status)
    }
}
