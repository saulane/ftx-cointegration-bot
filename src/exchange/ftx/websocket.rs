use serde_json::{Value, json};
use serde::{Deserialize, Serialize};

#[path = "utils.rs"]
mod utils;


#[derive(Serialize, Deserialize)]
pub struct TickerMessageData{
    bid: f64,
    ask: f64,
    bidSize: f64,
    askSize: f64,
    pub last: f64,
    time: f64
}

pub fn subscribe(channel: &str, market: Option<&str>) -> Value{
    let market = match market{
        Some(m) => m,
        None => "BTC-PERP"
    };

    let sub_msg = json!({
        "op": "subscribe",
        "channel": channel,
        "market": market
    });

    return sub_msg;
}

pub fn unsubscribe(channel: &str, market: &str) -> Value{
    let sub_msg = json!({
        "op": "unsubscribe",
        "channel": channel,
        "market": market
    });

    return sub_msg;
}

pub fn auth_msg(api_key: &str, api_secret: &str) -> Value{
    let (sign, ts) = utils::signature(api_secret, "ws", "ws", "ws").unwrap();
    let timestamp: u64 = ts.parse::<u64>().unwrap();
    let auth_msg = json!({
        "op": "login", 
        "args": {
            "key": api_key, 
            "sign": sign, 
            "time": timestamp,
        }
    });

    println!("{:?}", timestamp);

    return auth_msg;
}

pub fn data_msg(data_obj: String) -> Result<TickerMessageData, serde_json::Error>{

    let data:TickerMessageData = serde_json::from_str(&data_obj)?;

    Ok(data)
}