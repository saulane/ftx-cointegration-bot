extern crate reqwest;

use tungstenite::{connect, Message};
use url::Url;
use serde_json::{json, Value};


fn subscribe(channel: &str) -> Value{
    let sub_msg = json!({
        "op": "subscribe",
        "channel": channel,
        "market": "BTC-PERP"
    });

    return sub_msg;
}

async fn get_data() -> Result<(), Box<dyn std::error::Error>>{
    let rest_client = reqwest::Client::new();
    let res = rest_client.get("https://ftx.com/api/markets/BTC-PERP/candles?resolution=300&limit=20")
        .send()
        .await?
        .text()
        .await?;

    let body_json: Value = serde_json::from_str(&res).expect("Can't parse REST API request to JSON");

    println!("{:?}", body_json);

    Ok(())
}

#[tokio::main]
async fn main() {

    let _test = get_data().await;
    // println!("{:?}", test);

    let (mut socket, response) =
        connect(Url::parse("wss://ftx.com/ws/").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    let subcribe_msg  = subscribe("ticker");

    socket.write_message(Message::Text(subcribe_msg.to_string())).unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            _ => { panic!() }
        };
        let parsed: Value = serde_json::from_str(&msg).expect("Can't parse to JSON");

        println!("Last price: {:?}$", parsed["data"]["last"]);
    }
    // socket.close(None);
}



