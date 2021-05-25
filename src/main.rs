extern crate reqwest;
#[macro_use]
extern crate lazy_static;

use csv::StringRecord;
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
use std::env;
use std::fs;
use std::error::Error;


lazy_static!{
    static ref API_KEY: String = std::env::var("FTX_API_KEY").unwrap();
    static ref API_SECRET: String = std::env::var("FTX_API_SECRET").unwrap();
}

fn current_ts() -> u128 {
    let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    ts
}

// #[derive(Debug, Deserialize)]
// struct PairsCsv {
//     crypto1: String,
//     crypto2: String,
// }

fn pairs_reader() -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let file = fs::File::open("Cointegrated Pairs.txt").expect("Pls copy your 'Cointegrated Pairs.txt' in the root folder of this app");
    let mut rdr = csv::Reader::from_reader(file);
    let mut pairs_vec = Vec::new();
    for result in rdr.records() {
        let record = result?;
        pairs_vec.push(record);
    }
    Ok(pairs_vec)
}

#[tokio::main]
async fn main(){

    let pairs_from_file = pairs_reader().unwrap();
    let mut pair_symbol_list: Vec<[&str; 2]> = Vec::new();
    let mut symbol_list: Vec<&str> = Vec::new();


    for i in pairs_from_file.iter(){
        pair_symbol_list.push([&i[0], &i[1]]);

        if symbol_list.iter().any(|&j| j ==&i[0]){
            println!("Crypto Already in data");
        }else{
            symbol_list.push(&i[0]);
        }
    }

    println!("PairsList: {:?}, Number of pairs: {}", &pair_symbol_list,&pair_symbol_list.len());
    println!("CryptoList: {:?}, total in data: {}", &symbol_list, &symbol_list.len());

    let ftx_bot =  FtxApiClient{
        api_key: API_KEY.to_string(),
        api_secret: API_SECRET.to_string(),
        request_client: reqwest::Client::new()
    };

    let balance = ftx_bot.get_balance().await.unwrap();
    println!("{:?}", balance);


    // let mut symbol_last_prices_list = HashMap::new();
    let mut pair_list: Vec<Pair> = Vec::new();

    for i in pair_symbol_list{
        let crypto1:HistoricalData = HistoricalData::new(ftx_bot.fetch_historical_data(i[0], "900").await.unwrap()).unwrap();
        let crypto2:HistoricalData = HistoricalData::new(ftx_bot.fetch_historical_data(i[1], "900").await.unwrap()).unwrap();

        let pair_tmp: Pair = Pair::new(i[0].to_string(),&crypto1, i[1].to_string(),&crypto2);
        println!("Pair: {}{} added", &i[0].to_string(),&i[1].to_string());
        pair_list.push(pair_tmp);
    }



    //println!("{:?}", &pair_list);


    // let (mut socket, response) = connect(Url::parse("wss://ftx.com/ws/").unwrap()).expect("Can't connect");

    // println!("Connected to the server");
    // println!("Response HTTP code: {}", response.status());

    
    // let auth_msg = ws::auth_msg(&API_KEY.to_string(), &API_SECRET.to_string());
    
    
    // let sub_msgs = ws::subscribe("ticker", Some(&symbol_list));
    
    // socket.write_message(Message::Text(auth_msg.to_string())).unwrap();
    // socket.write_message(Message::Text(ws::subscribe("orders", None)[0].to_string())).unwrap();
    // socket.write_message(Message::Text(ws::subscribe("fills", None)[0].to_string())).unwrap();
    
    // if current_ts().to_string().ends_with("5000"){
    //     socket.write_message(Message::Text("{'op': 'ping'}".to_string())).unwrap();
    // }
    
    // for s in sub_msgs{
    //     socket.write_message(Message::Text(s.to_string())).unwrap();
    // }
    // println!("Subscribed to tickers");


    loop {
        // let msg = socket.read_message().expect("Error reading message");
        // let msg = match msg {
        //     tungstenite::Message::Text(s) => { s }
        //     _ => { panic!() }
        // };
        // let parsed: Value = serde_json::from_str(&msg).expect("Can't parse message to JSON");
        
        // if parsed["channel"] == "ticker" && parsed["type"] == "update"{
        //     //println!("{:?}", &parsed);
        //     let tickermessage: ws::TickerMessage = serde_json::from_value(parsed).expect("Problem parsing JSON");
        //     let data_obj: ws::TickerMessageData = tickermessage.data;
        //     symbol_last_prices_list.insert(tickermessage.market, data_obj.last);
        // };

        
        for p in pair_list.iter_mut(){
            // for s in symbol_list.iter(){
            //     if symbol_last_prices_list.contains_key(&s.to_string()){
            //         //println!("{}: {}",&s.to_string(),symbol_last_prices_list[&s.to_string()]);
            //         if p.pair[0] == s.to_string(){
            //             p.crypto_1[19] = symbol_last_prices_list[&s.to_string()];
            //         }else if p.pair[1] == s.to_string(){
            //             p.crypto_2[19] = symbol_last_prices_list[&s.to_string()];
            //         }
            //     }
            // }

            p.update_zscore();

            // if current_ts() as f64 >= p.last_bar_ts+910000.0{
            //     let neweth = HistoricalData::new(ftx_bot.fetch_historical_data(&p.pair[0], "900").await.unwrap()).unwrap();
            //     let newltc = HistoricalData::new(ftx_bot.fetch_historical_data(&p.pair[1], "900").await.unwrap()).unwrap();
    
            //     p.update_prices(&neweth, &newltc);
            //     println!("New OHLC Fetched");

            //     std::thread::sleep(std::time::Duration::from_millis(100));
            // }
            let neweth = HistoricalData::new(ftx_bot.fetch_historical_data(&p.pair[0], "900").await.unwrap()).unwrap();
            let newltc = HistoricalData::new(ftx_bot.fetch_historical_data(&p.pair[1], "900").await.unwrap()).unwrap();

            p.update_prices(&neweth, &newltc);
            // println!("New OHLC Fetched");

            std::thread::sleep(std::time::Duration::from_millis(300));
            

            if p.zscore.abs() >= 1.5{
                println!("Pair: {}/{}, Zscore: {} , Opportuniy ? ->  {:?} | ",&p.pair[0], &p.pair[1], &p.zscore,&p.position_size(&balance.get_USD_Balance()[0], &balance.get_USD_Balance()[1]));
            }else{
                //println!("Pair: {}/{}, Zscore: {}",&p.pair[0], &p.pair[1], &p.zscore);
            }
            // //print!("{}: {} , {}: {} | ", &p.pair[0],&p.crypto_1.last().unwrap(),&p.pair[1], &p.crypto_2.last().unwrap());
            

            
        }



    }
    // socket.close(None);
}



