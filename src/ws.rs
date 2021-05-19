use serde_json::{Value, json};

pub fn subscribe(channel: &str) -> Value{
    let sub_msg = json!({
        "op": "subscribe",
        "channel": channel,
        "market": "BTC-PERP"
    });

    return sub_msg;
}

pub fn unsubscribe(channel: &str) -> Value{
    let sub_msg = json!({
        "op": "unsubscribe",
        "channel": channel,
        "market": "BTC-PERP"
    });

    return sub_msg;
}
