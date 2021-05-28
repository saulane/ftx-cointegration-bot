use serde_json::{Value, json};
use serde::{Deserialize, Serialize};

use crate::lib::{utils};

#[derive(Serialize, Deserialize)]
pub struct TickerMessageData{
    pub bid: f64,
    pub ask: f64,
    pub bidSize: f64,
    pub askSize: f64,
    pub last: f64,
    pub time: f64
}
#[derive(Serialize, Deserialize)]
pub struct TickerMessage{
    pub channel: String,
    pub market: String,
    pub r#type: String,
    pub data: TickerMessageData,
}

pub fn subscribe(channel: &str, markets: Option<&Vec<&str>>) -> Vec<Value>{
    let mut msgs:Vec<Value> = Vec::new();

    match markets{
        Some(m) if m.len()>0 => {
            for i in m.iter(){
                let sub_msg: Value = json!({
                    "op": "subscribe",
                    "channel": channel,
                    "market": i
                });
                msgs.push(sub_msg);
            }
        },
        _ => {
            let sub_msg: Value = json!({
                "op": "subscribe",
                "channel": channel,
                "market": "BTC-PERP"
            });
            msgs.push(sub_msg);
        }
    };

    return msgs;
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