extern crate hex;
use std::time::{SystemTime};
use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};

use csv::StringRecord;
use tungstenite::protocol::frame::coding::Data;
use std::fs;
use std::error::Error;
use std::collections::HashMap;

use super::data_type::{Pair, HistoricalData, DataFile, Trade};
use super::rest_api::FtxApiClient;
use super::coint::coint;

const MAX_POS: u32 = 4;

pub fn current_ts() -> u128 {
    let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    ts
}

pub fn signature(api_secret: &str, endpoint: &str, method: &str, api: &str) -> Result<(String, String), ()> {
    type HmacSha256 = Hmac<Sha256>;
    let ts: String = current_ts().to_string();

    let message = if api=="rest" {format!("{}{}/api{}", ts, method, endpoint)} else {format!("{}websocket_login", ts)};
    //println!("URL payload: {}", message);

    let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes()).expect("Problem keying the API_SECRET");
    mac.update(&message.as_bytes());
    let result = mac.finalize().into_bytes();

    let r2 = hex::encode(&result);

    Ok((r2, ts))
}

pub fn pairs_reader() -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let file = fs::File::open("CointPairs.txt").expect("Pls copy your 'CointPairs.txt' in the root folder of this app");
    let mut rdr = csv::Reader::from_reader(file);
    let mut pairs_vec = Vec::new();
    for result in rdr.records() {
        let record = result?;
        pairs_vec.push(record);
    }
    Ok(pairs_vec)
}

pub async fn make_pair_list(pair_symbol_list: &Vec<[String; 2]>, ftx_bot: &FtxApiClient) -> Result<Vec<Pair>, ()>{
    let mut pair_list: Vec<Pair> = Vec::new(); 
    for i in pair_symbol_list{
        let mut i_counter: u32 = 0;
        'datafetching: loop{
            if i_counter != 0{
                std::thread::sleep(std::time::Duration::from_millis(20));
            }

            i_counter = i_counter+1;

            let (c1_data,c2_data) =  match (ftx_bot.fetch_historical_data(&i[0], "900", "20").await, ftx_bot.fetch_historical_data(&i[1], "900", "20").await){
                (Ok(c1), Ok(c2)) => (c1, c2),
                _ => continue 'datafetching
            };

            let (crypto1, crypto2) = match (HistoricalData::new(c1_data), HistoricalData::new(c2_data)){
                (Ok(c1), Ok(c2)) => (c1, c2),
                _ => continue 'datafetching
            };

            let pair_tmp: Pair = match Pair::new(i[0].to_string(),&crypto1, i[1].to_string(),&crypto2, MAX_POS){
                Ok(newpair) => newpair,
                _ => continue 'datafetching
            };

            //println!("Pair: {}{} added", &i[0].to_string(),&i[1].to_string());
            pair_list.push(pair_tmp);
            break 'datafetching;
        }

        std::thread::sleep(std::time::Duration::from_millis(40));
    }
    return Ok(pair_list)
}


pub async fn coint_pairs_list(ftx_bot: &FtxApiClient) -> Result<(Vec<[String;2]>, Vec<String>), Box<dyn std::error::Error>>{
    let crypto_list = ftx_bot.get_markets_filtered().await.unwrap();
    let mut crypto_data: HashMap<String, HistoricalData> = HashMap::new();

    for i in &crypto_list{
        crypto_data.insert(i.to_string(), HistoricalData::new(ftx_bot.fetch_historical_data(i, "900", "1000").await.unwrap()).unwrap());
    }

    let mut possible_pairs:Vec<[String; 2]> = Vec::new();
    for i in &crypto_list{
        for j in &crypto_list{
            if i != j && !possible_pairs.contains(&[j.to_string(), i.to_string()]){
                possible_pairs.push([i.to_string(),j.to_string()]);
            }
        }
    }

    println!("Number of pairs created: {:?}",&possible_pairs.len());

    let mut coint_pairs:Vec<[String; 2]> = Vec::new();
    let mut used_crypto:Vec<String> = Vec::new();
    
    for i in possible_pairs{
        // println!("Pairs: {:?}",&i);
        let test_coint = match coint(&crypto_data.get(&i[0]).unwrap().prices().unwrap(), &crypto_data.get(&i[1]).unwrap().prices().unwrap()){
            Ok(val) => val,
            _ => continue,
        };
        if test_coint{
            
            if !used_crypto.contains(&i[0]){
                used_crypto.push(i[0].to_string());
            }

            if !used_crypto.contains(&i[1]){
                used_crypto.push(i[1].to_string());
            }

            coint_pairs.push(i);
        }   
    }

    let path = std::path::Path::new("data.json");
    let read_file = fs::OpenOptions::new()
        .read(true)
        .open(path).expect("Unable to read file");

    let res: DataFile = serde_json::from_reader(&read_file).expect("Unable to open file");

    let write_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path).expect("Unable to write file");

    let new_data: DataFile = DataFile{last_update: current_ts(), trades: res.trades};
    serde_json::to_writer_pretty(&write_file, &new_data);


    println!("Number of cointegrated pairs: {:?}",&coint_pairs.len());
    Ok((coint_pairs, used_crypto))
}

pub fn save_trade(pair: &String, profit: f64){

    let path = std::path::Path::new("data.json");
    let read_file = fs::OpenOptions::new()
        .read(true)
        .open(path).expect("Unable to read file");

    let mut res: DataFile = serde_json::from_reader(&read_file).expect("Unable to open file");
    res.trades.push(Trade{pair: pair.to_string(), profit: profit});

    let write_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path).expect("Unable to write file");


    serde_json::to_writer_pretty(&write_file, &res);
}

pub fn read_last_pairs_update() -> u128{
    let path = std::path::Path::new("data.json");

    let data = fs::File::open(path).expect("Unable to read file");
    let res:DataFile = serde_json::from_reader(&data).expect("Unable to serialize data");

    res.last_update
}