extern crate reqwest;
#[macro_use]
extern crate lazy_static;

use tungstenite::{connect, Message};
use url::Url;
use serde_json::Value;


mod rest_api;
mod ws;
mod utils;
mod strategy;

// use rest_api::FtxApiClient;



lazy_static!{
    static ref API_KEY: String = std::env::var("FTX_API_KEY").unwrap();
    static ref API_SECRET: String = std::env::var("FTX_API_SECRET").unwrap();
}

#[tokio::main]
async fn main(){
    
    // let ftx_bot =  FtxApiClient{
    //     api_key: API_KEY.to_string(),
    //     api_secret: API_SECRET.to_string(),
    //     request_client: reqwest::Client::new()
    // };

    let mut market_state: strategy::MarketState = strategy::MarketState::new();

    let mut btc_price: f64 = 0.0;
    let mut bch_price: f64 = 0.0; 

    // let test = ftx_bot.fetch_historical_data("BTC-PERP", "300").await.unwrap();
    // println!("{:?}", test);

    let (mut socket, response) =
        connect(Url::parse("wss://ftx.com/ws/").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    
    let auth_msg = ws::auth_msg(&API_KEY.to_string(), &API_SECRET.to_string());
    // let subcribe_msg  = ws::subscribe("orders");
    
    socket.write_message(Message::Text(auth_msg.to_string())).unwrap();
    socket.write_message(Message::Text(ws::subscribe("orders", None).to_string())).unwrap();
    socket.write_message(Message::Text(ws::subscribe("fills", None).to_string())).unwrap();
    socket.write_message(Message::Text(ws::subscribe("ticker", Some("BTC/USD")).to_string())).unwrap();
    socket.write_message(Message::Text(ws::subscribe("ticker", Some("BCH/USD")).to_string())).unwrap();

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            _ => { panic!() }
        };
        let parsed: Value = serde_json::from_str(&msg).expect("Can't parse message to JSON");

        if parsed["channel"] == "ticker"{
            let data_obj = parsed["data"].to_string();
            if data_obj != "null"{
                if parsed["market"] == "BTC/USD"{
                    let data = ws::data_msg(data_obj).unwrap();
                    btc_price = data.last;
                    // println!("BTC: {}", init_btc_price);
                }else if parsed["market"] == "BCH/USD"{
                    let data = ws::data_msg(data_obj).unwrap();
                    bch_price = data.last;
                    // println!("BCH: {}", init_bch_price);
                }
            }
        };

        if bch_price != 0.0 && btc_price != 0.0 && utils::current_ts()-market_state.last_update_ts>=500{
            market_state.update_price(btc_price, bch_price);

            println!("{:?}", market_state);
        }

        

    }
    // socket.close(None);
}



