extern crate reqwest;
#[macro_use]
extern crate lazy_static;

use tungstenite::{connect, Message};
use url::Url;
use serde_json::Value;


mod exchange;
mod strategy;

use std::collections::HashMap;
use std::time::{SystemTime};
use std::vec;
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

    let balance = ftx_bot.get_balance().await.unwrap();
    println!("{:?}", balance);

    let pair_symbol_list = vec!(["DOGE-PERP", "VET-PERP"]);
    let symbol_list = vec!("DOGE-PERP", "VET-PERP");
    let mut symbol_last_prices_list = HashMap::new();
    let mut pair_list: Vec<Pair> = Vec::new();

    for i in pair_symbol_list{
        let crypto1:HistoricalData = HistoricalData::new(ftx_bot.fetch_historical_data(i[0], "900").await.unwrap()).unwrap();
        let crypto2:HistoricalData = HistoricalData::new(ftx_bot.fetch_historical_data(i[1], "900").await.unwrap()).unwrap();

        let pair_tmp: Pair = Pair::new(i[0].to_string(),&crypto1, i[1].to_string(),&crypto2);
        pair_list.push(pair_tmp);
    }



    println!("{:?}", &pair_list);


    let (mut socket, response) = connect(Url::parse("wss://ftx.com/ws/").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    
    let auth_msg = ws::auth_msg(&API_KEY.to_string(), &API_SECRET.to_string());
    
    
    let sub_msgs = ws::subscribe("ticker", Some(&symbol_list));
    
    socket.write_message(Message::Text(auth_msg.to_string())).unwrap();
    socket.write_message(Message::Text(ws::subscribe("orders", None)[0].to_string())).unwrap();
    socket.write_message(Message::Text(ws::subscribe("fills", None)[0].to_string())).unwrap();
    
    
    for s in sub_msgs{
        socket.write_message(Message::Text(s.to_string())).unwrap();
    }


    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            _ => { panic!() }
        };
        let parsed: Value = serde_json::from_str(&msg).expect("Can't parse message to JSON");
        // println!("{}", parsed);
        if parsed["channel"] == "ticker" && parsed["type"] == "update"{
            let tickermessage: ws::TickerMessage = serde_json::from_value(parsed).expect("Problem parsing JSON");
            let data_obj: ws::TickerMessageData = tickermessage.data;
            symbol_last_prices_list.insert(tickermessage.market, data_obj.last);
        };

        
        for p in pair_list.iter_mut(){
            for s in symbol_list.iter(){
                if symbol_last_prices_list.contains_key(&s.to_string()){
                    //println!("{}: {}",&s.to_string(),symbol_last_prices_list[&s.to_string()]);
                    if p.pair[0] == s.to_string(){
                        p.crypto_1[19] = symbol_last_prices_list[&s.to_string()];
                    }else if p.pair[1] == s.to_string(){
                        p.crypto_2[19] = symbol_last_prices_list[&s.to_string()];
                    }
                }
            }

            p.update_zscore();

            if current_ts() as f64 >= p.last_bar_ts+910000.0{
                let neweth = HistoricalData::new(ftx_bot.fetch_historical_data(&p.pair[0], "900").await.unwrap()).unwrap();
                let newltc = HistoricalData::new(ftx_bot.fetch_historical_data(&p.pair[1], "900").await.unwrap()).unwrap();
    
                p.update_prices(&neweth, &newltc);
                println!("New OHLC Fetched");
            }

            print!("{}: {} | {}: {}", &p.pair[0],&p.crypto_1.last().unwrap(),&p.pair[1], &p.crypto_2.last().unwrap());
            print!("Zscore: {} | Opportuniy ? ->  {:?}", &p.zscore,&p.position_size(&balance.get_USD_Balance()[0], &balance.get_USD_Balance()[1]));
            
            println!("{:?}", current_ts());

            
        }



    }
    // socket.close(None);
}



