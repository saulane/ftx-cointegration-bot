extern crate serde_json;

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

fn main() {
    env_logger::init();

    let (mut socket, response) =
        connect(Url::parse("wss://ftx.com/ws/").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }



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



