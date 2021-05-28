extern crate reqwest;
#[macro_use]
extern crate lazy_static;

mod exchange;
mod strategy;
mod lib;

use lib::utils::current_ts;

use csv::StringRecord;

use std::collections::HashMap;
use exchange::ftx::rest_api::FtxApiClient;
use strategy::data_type::{HistoricalData, Pair, Position};
use strategy::python_script;
use std::fs;
use std::error::Error;
use chrono::{Utc, Timelike};

lazy_static!{
    static ref API_KEY: String = std::env::var("FTX_API_KEY").unwrap();
    static ref API_SECRET: String = std::env::var("FTX_API_SECRET").unwrap();
    
}

static mut STOP:bool = false;
static MAX_POS: u32 = 3;

fn pairs_reader() -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let file = fs::File::open("CointPairs.txt").expect("Pls copy your 'CointPairs.txt' in the root folder of this app");
    let mut rdr = csv::Reader::from_reader(file);
    let mut pairs_vec = Vec::new();
    for result in rdr.records() {
        let record = result?;
        pairs_vec.push(record);
    }
    Ok(pairs_vec)
}

fn check_timing() -> bool{
    let curr_minute = Utc::now().minute();
    let end_range = 12..16;

    if curr_minute % 15 <= 3 || end_range.contains(&curr_minute){
        return true
    }else{
        return false
    }
}

async fn make_pair_list(pair_symbol_list: &Vec<[&str; 2]>, ftx_bot: &FtxApiClient) -> Result<Vec<Pair>, ()>{
   let mut pair_list: Vec<Pair> = Vec::new(); 
    for i in pair_symbol_list{
        let mut i_counter = 0;
        'datafetching: loop{
            if i_counter != 0{
                std::thread::sleep(std::time::Duration::from_millis(60));
            }

            i_counter = i_counter+1;

            let (c1_data,c2_data) =  match (ftx_bot.fetch_historical_data(i[0], "900").await, ftx_bot.fetch_historical_data(i[1], "900").await){
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

        std::thread::sleep(std::time::Duration::from_millis(60));
    }
    return Ok(pair_list)
}



#[tokio::main]
async fn main(){


    let ftx_bot =  FtxApiClient{
        api_key: API_KEY.to_string(),
        api_secret: API_SECRET.to_string(),
        request_client: reqwest::Client::new()
    };


    ctrlc::set_handler(move || {
        unsafe{
            STOP = true;
            println!("Stopping Program!");
        }
    })
    .expect("Error setting Ctrl-C handler");

    let pairs_from_file = pairs_reader().unwrap();

    //Pushing every pair in a Vec
    let mut pair_symbol_list: Vec<[&str; 2]> = Vec::new();
    let mut symbol_list: Vec<&str> = Vec::new();
    for i in pairs_from_file.iter(){
        pair_symbol_list.push([&i[0], &i[1]]);

        if !symbol_list.iter().any(|&j| j ==&i[0]){
            symbol_list.push(&i[0]);
            
        }
    }
    let mut pair_list: Vec<Pair> = make_pair_list(&pair_symbol_list, &ftx_bot).await.unwrap();
    let mut positions: HashMap<String, Position> = HashMap::new();


    let mut last_ohlc_update = current_ts();
    let mut last_coint_test_update = current_ts();


    println!("PairsList: {:?}, Number of pairs: {}", &pair_symbol_list,&pair_symbol_list.len());
    println!("CryptoList: {:?}, total in data: {}", &symbol_list, &symbol_list.len());


    'mainloop: loop {
        
        if current_ts()-last_ohlc_update >= 60000 {
            println!("All OHLC updated!");
            pair_list = make_pair_list(&pair_symbol_list, &ftx_bot).await.unwrap();

            last_ohlc_update = current_ts();
        }

        if positions.len() == 0 {
            if current_ts() - last_coint_test_update >= 864000000 {
                python_script::get_new_coint_pairs().await;

                last_coint_test_update = current_ts();
            }
        }

        'pairsloop: for p in pair_list.iter_mut(){
            unsafe{
                if STOP && positions.len() == 0{
                    break 'mainloop;
                }
            }


            let crypto1 = ftx_bot.get_market(&p.pair[0]).await;
            let crypto2 = ftx_bot.get_market(&p.pair[1]).await;

            match (crypto1, crypto2){
                (Ok(c1), Ok(c2)) => {
                    match p.update_last_prices(c1.last, c2.last){
                        Ok(()) => print!(""),
                        _ => continue 'pairsloop
                    };
                },
                _=> continue 'pairsloop
            };
            


            

            std::thread::sleep(std::time::Duration::from_millis(60));
            

            if positions.contains_key(&p.pair_id){
                if p.zscore.abs() <= 0.5{            
                    println!("Closing Trade on pair: {}, zscore={}", &p.pair_id, &p.zscore);
                    let curr_crypto1_pos_size = -&positions[&p.pair_id].crypto1_size;
                    let curr_crypto2_pos_size = -&positions[&p.pair_id].crypto2_size;

                    // let close_order1 = ftx_bot.post_order(&p.pair[0],0.0, curr_crypto1_pos_size, "market", true).await;
                    // let close_order2 = ftx_bot.post_order(&p.pair[1],0.0, curr_crypto2_pos_size, "market", true).await;

                    // match (close_order1, close_order2){
                    //     (Ok(res1), Ok(res2)) => {
                    //         positions.remove(&p.pair_id);
                    //     },
                    //     _ => println!("Problem closing position on exchange")
                    // }
                }else{
                    println!("Open Position on {}, zscore = {}", &p.pair_id, &p.zscore);
                }
            }else{
                unsafe{
                    if p.zscore.abs()>=1.5 && positions.len() < MAX_POS as usize && STOP == false{ 
                        println!("New opportunity found!: {:?}", &p.pair);
                        let curr_balance = ftx_bot.get_balance().await.unwrap();
                        // match &p.position_size(&curr_balance.get_USD_Balance()[0], &curr_balance.get_USD_Balance()[1]){
                        //     Some(size) => {
                        //         let order1 = ftx_bot.post_order(&p.pair[0],0.0, size[0], "market", false).await;
                        //         let order2 = ftx_bot.post_order(&p.pair[1],0.0, size[1], "market", false).await;

                        //         match (order1, order2){
                        //             (Ok(res1), Ok(res2)) => {
                        //                 let new_pos = Position::new([p.pair[0].to_string(), p.pair[1].to_string()], *p.crypto_1.last().unwrap(), *p.crypto_2.last().unwrap(), p.zscore, size[0], size[1]);
                        //                 positions.insert(p.pair_id.to_string(), new_pos);
                        //             },
                        //             _ => println!("Error posting order to api")
                        //         }
                        //     },
                        //     _ => println!("Not enough money!"),
                        // }
                    }else{
                        println!("Pair: {}, Zscore: {}",&p.pair_id, &p.zscore);
                    }
                }
            }   
        }
    }

    println!("Program stopped");
}
