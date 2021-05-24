extern crate reqwest;
#[macro_use]
extern crate lazy_static;

use tungstenite::{connect, Message};
use url::Url;
use serde_json::Value;


mod exchange;
mod strategy;

use std::time::{SystemTime};
use exchange::ftx::rest_api::FtxApiClient;
use exchange::ftx::websocket as ws;
use strategy::data_type::{HistoricalData, Pair};


lazy_static!{
    static ref API_KEY: String = std::env::var("FTX_API_KEY").unwrap();
    static ref API_SECRET: String = std::env::var("FTX_API_SECRET").unwrap();
}

fn current_ts() -> u128 {
    let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    ts
}


#[tokio::main]
async fn main(){
    let ftx_bot =  FtxApiClient{
        api_key: API_KEY.to_string(),
        api_secret: API_SECRET.to_string(),
        request_client: reqwest::Client::new()
    };



    let eth:HistoricalData = HistoricalData::new(ftx_bot.fetch_historical_data("ETH/USD", "900").await.unwrap()).unwrap();
    // println!("{:?}", &eth);
    let ltc:HistoricalData = HistoricalData::new(ftx_bot.fetch_historical_data("LTC/USD", "900").await.unwrap()).unwrap();
    // println!("{:?}", &ltc);

    let mut pair: Pair = Pair::new(&eth, &ltc);


    println!("{:?}", &pair);

    // loop{
    //     if current_ts()>= last_update+2000{
    //         eth = HistoricalData::new(ftx_bot.fetch_historical_data("ETH/USD", "900").await.unwrap()).unwrap();
    //         ltc = HistoricalData::new(ftx_bot.fetch_historical_data("LTC/USD", "900").await.unwrap()).unwrap();

    //         pair = Pair::new(&eth, &ltc);

    //         println!("ETH: {} | LTC: {:?}", &pair.crypto_1[19], &pair.crypto_2[19]);
    //         println!("Zscore: {} | Opportuniy ? ->  {:?}", &pair.zscore,&pair.decision_making().unwrap());
    //         println!("{:?}", current_ts());
    //         last_update = current_ts();
    //     }
    // }




    let (mut socket, response) = connect(Url::parse("wss://ftx.com/ws/").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    
    let auth_msg = ws::auth_msg(&API_KEY.to_string(), &API_SECRET.to_string());
    // let subcribe_msg  = ws::subscribe("orders");
    
    socket.write_message(Message::Text(auth_msg.to_string())).unwrap();
    socket.write_message(Message::Text(ws::subscribe("orders", None).to_string())).unwrap();
    socket.write_message(Message::Text(ws::subscribe("fills", None).to_string())).unwrap();
    socket.write_message(Message::Text(ws::subscribe("ticker", Some("ETH/USD")).to_string())).unwrap();
    socket.write_message(Message::Text(ws::subscribe("ticker", Some("LTC/USD")).to_string())).unwrap();

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
                if parsed["market"] == "ETH/USD"{
                    let data = ws::data_msg(data_obj).unwrap();
                    pair.crypto_1[19] = data.last;
                }else if parsed["market"] == "LTC/USD"{
                    let data = ws::data_msg(data_obj).unwrap();
                    pair.crypto_2[19] = data.last;
                }
            }
        };

        pair.update_zscore();
        println!("ETH: {} | LTC: {:?}", &pair.crypto_1.last().unwrap(), &pair.crypto_2.last().unwrap());
        println!("ETH_array: {:?}", &pair.crypto_1);
        println!("Zscore: {} | Opportuniy ? ->  {:?}", &pair.zscore,&pair.decision_making().unwrap());
        
        println!("{:?}", current_ts());

        if current_ts() as f64 >= pair.last_bar_ts+910000.0{
            let neweth = HistoricalData::new(ftx_bot.fetch_historical_data("ETH/USD", "900").await.unwrap()).unwrap();
            let newltc = HistoricalData::new(ftx_bot.fetch_historical_data("LTC/USD", "900").await.unwrap()).unwrap();

            pair.update_prices(&neweth, &newltc);
            println!("New OHLC Fetched");
        }

    }
    // socket.close(None);
}



