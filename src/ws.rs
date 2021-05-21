use serde_json::{Value, json};

#[path = "utils.rs"]
mod utils;

pub fn subscribe(channel: &str, market: Option<&str>) -> Value{
    let mut market = match market{
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