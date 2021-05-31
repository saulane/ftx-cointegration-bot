extern crate reqwest;
#[macro_use]
extern crate lazy_static;

mod exchange;
mod strategy;
mod lib;

use lib::utils::pairs_reader;
use strategy::coint::coint;

use std::io::Write;
use std::fs::OpenOptions;

use std::collections::HashMap;
use exchange::ftx::rest_api::FtxApiClient;
use strategy::data_type::{HistoricalData, Pair, Position};

lazy_static!{
    static ref API_KEY: String = std::env::var("FTX_API_KEY").unwrap();
    static ref API_SECRET: String = std::env::var("FTX_API_SECRET").unwrap();
    
}

static mut STOP:bool = false;
static MAX_POS: u32 = 3;

pub fn is_used(crypto: &str,positions:&HashMap<String, Position>) -> bool{
    for k in positions.keys(){
        if k.contains(crypto){
            return true;
        }
    }
    return false;
}

pub async fn make_pair_list(pair_symbol_list: &Vec<[&str; 2]>, ftx_bot: &FtxApiClient) -> Result<Vec<Pair>, ()>{
    let mut pair_list: Vec<Pair> = Vec::new(); 
    for i in pair_symbol_list{
        let mut i_counter = 0;
        'datafetching: loop{
            if i_counter != 0{
                std::thread::sleep(std::time::Duration::from_millis(20));
            }

            i_counter = i_counter+1;

            let (c1_data,c2_data) =  match (ftx_bot.fetch_historical_data(i[0], "900", "20").await, ftx_bot.fetch_historical_data(i[1], "900", "20").await){
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


async fn coint_pairs_list(ftx_bot: &FtxApiClient) -> (Vec<[String;2]>, Vec<String>){
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
        if coint(&crypto_data.get(&i[0]).unwrap().prices().unwrap(), &crypto_data.get(&i[1]).unwrap().prices().unwrap()){
            
            if !used_crypto.contains(&i[0]){
                used_crypto.push(i[0].to_string());
            }

            if !used_crypto.contains(&i[1]){
                used_crypto.push(i[1].to_string());
            }

            coint_pairs.push(i);
        }   
    }

    println!("Number of cointegrated pairs: {:?}",&coint_pairs.len());
    (coint_pairs, used_crypto)
}



#[tokio::main]
async fn main(){
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");

    let ftx_bot =  FtxApiClient{
        api_key: API_KEY.to_string(),
        api_secret: API_SECRET.to_string(),
        request_client: reqwest::Client::new()
    };

    // println!("Coint test:{:?}", coint(
    //     &HistoricalData::new(ftx_bot.fetch_historical_data("BTC-PERP", "900", "1000").await.unwrap()).unwrap().prices().unwrap(),
    //     &HistoricalData::new(ftx_bot.fetch_historical_data("ETH-PERP", "900", "1000").await.unwrap()).unwrap().prices().unwrap()
    // ));


    println!("Pairs Coint:{:?}", coint_pairs_list(&ftx_bot).await);


    //listening to CTRL-C in order to stop the program
    ctrlc::set_handler(move || {
        unsafe{
            if STOP == true{
                println!("Forcing Close");
                std::process::exit(0);
            }
            STOP = true;
            println!("Stopping Program!");

        }
    })
    .expect("Error setting Ctrl-C handler");

    let pairs_from_file = pairs_reader().unwrap();

    let pairs_coint_test = coint_pairs_list(&ftx_bot).await;

    //Pushing every pair in a Vec
    let mut pair_symbol_list: Vec<[&str; 2]> = Vec::new();
    let mut symbol_list: Vec<&str> = Vec::new();
    for i in pairs_coint_test.0.iter(){
        pair_symbol_list.push([&i[0], &i[1]]);

        if !symbol_list.iter().any(|&j| j == &i[0]){
            symbol_list.push(&i[0]);  
        }

        if !symbol_list.iter().any(|&j| j == &i[1]){
            symbol_list.push(&i[1]);  
        }
    }
    let mut pair_list: Vec<Pair> = make_pair_list(&pair_symbol_list, &ftx_bot).await.unwrap();
    let mut positions: HashMap<String, Position> = HashMap::new();

    //let mut last_coint_test_update = current_ts();


    println!("PairsList: {:?}, Number of pairs: {}", &pair_symbol_list,&pair_symbol_list.len());
    println!("CryptoList: {:?}, total in data: {}", &symbol_list, &symbol_list.len());


    //Main Infinite Loop
    'mainloop: loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
        //Iterate over every tradable pairs
        'pairsloop: for p in pair_list.iter_mut(){
            unsafe{
                if STOP && positions.len() == 0{
                    break 'mainloop;
                }
            }

            let c1_hist = ftx_bot.fetch_historical_data(&p.pair[0], "900", "20").await;
            let c2_hist = ftx_bot.fetch_historical_data(&p.pair[1], "900", "20").await;

            match (c1_hist, c2_hist) {
                (Ok(c1h), Ok(c2h)) => {
                    match (HistoricalData::new(c1h),HistoricalData::new(c2h)){
                        (Ok(hist1), Ok(hist2)) => {
                            let _update_res = p.update_prices(&hist1, &hist2);
                        },
                        _ => continue 'pairsloop,
                    };
                },
                _ => continue 'pairsloop,
            };

            std::thread::sleep(std::time::Duration::from_millis(100));


            // let crypto1 = ftx_bot.get_market(&p.pair[0]).await;
            // let crypto2 = ftx_bot.get_market(&p.pair[1]).await;
            // match (crypto1, crypto2){
            //     (Ok(c1), Ok(c2)) => {
            //         match p.update_last_prices(c1.last, c2.last){
            //             Ok(()) => print!(""),
            //             _ => continue 'pairsloop
            //         };
            //     },
            //     _=> continue 'pairsloop
            // };
            // std::thread::sleep(std::time::Duration::from_millis(70));
            
            //Check if the Pair is already in position
            if positions.contains_key(&p.pair_id){
                unsafe{
                    if p.zscore.abs() <= 0.5 || STOP == true{            
                        println!("Closing Trade on pair: {}, zscore={}", &p.pair_id, &p.zscore);
                        let curr_crypto1_pos_size = -&positions[&p.pair_id].crypto1_size;
                        let curr_crypto2_pos_size = -&positions[&p.pair_id].crypto2_size;

                        let crypto1_profit = if &positions[&p.pair_id].crypto1_size >= &0.0 { p.crypto_1.last().unwrap()/&positions[&p.pair_id].crypto1_entry_price - 1.0 } else { &positions[&p.pair_id].crypto1_entry_price/p.crypto_1.last().unwrap() - 1.0 };
                        let crypto2_profit = if &positions[&p.pair_id].crypto2_size >= &0.0 { p.crypto_2.last().unwrap()/&positions[&p.pair_id].crypto2_entry_price - 1.0 } else { &positions[&p.pair_id].crypto2_entry_price/p.crypto_2.last().unwrap() - 1.0 };

                        let trade_res = format!("{}, {}\n", &p.pair_id, (&crypto1_profit+&crypto2_profit-0.07/100.0)*100.0);
                        file.write_all(trade_res.as_bytes()).expect("write failed");
                        //writeln!(&mut file, "{}", &trade_res).unwrap();
                        positions.remove(&p.pair_id);

                        // let close_order1 = ftx_bot.post_order(&p.pair[0],0.0, curr_crypto1_pos_size, "market", true).await;
                        // let close_order2 = ftx_bot.post_order(&p.pair[1],0.0, curr_crypto2_pos_size, "market", true).await;

                        // match (close_order1, close_order2){
                        //     (Ok(_res1), Ok(_res2)) => {
                        //         positions.remove(&p.pair_id);
                        //     },
                        //     _ => println!("Problem closing position on exchange")
                        // }
                    }else{
                        println!("Open Position on {}, zscore = {}", &p.pair_id, &p.zscore);
                    }
                }
            }else{
                unsafe{
                    
                    //Check if a crypto is already in use in order to reduce risk
                    let cryptos_names = p.pair_id.split("/");
                    for symbol in cryptos_names{
                        if is_used(symbol, &positions){
                            continue 'pairsloop;
                        }
                    }

                    //Check for opportunity
                    if p.zscore.abs()>=1.5 && positions.len() < MAX_POS as usize && STOP == false{ 
                        println!("New opportunity found!: {:?}", &p.pair);
                        let curr_balance = ftx_bot.get_balance().await.unwrap();
                        match &p.position_size(&curr_balance.get_usd_Balance()[0], &curr_balance.get_usd_Balance()[1]){
                            Some(size) => {
                                let new_pos = Position::new([p.pair[0].to_string(), p.pair[1].to_string()], *p.crypto_1.last().unwrap(), *p.crypto_2.last().unwrap(), p.zscore, size[0], size[1]);
                                positions.insert(p.pair_id.to_string(), new_pos);
                                // let order1 = ftx_bot.post_order(&p.pair[0],0.0, size[0], "market", false).await;
                                // let order2 = ftx_bot.post_order(&p.pair[1],0.0, size[1], "market", false).await;

                                // match (order1, order2){
                                //     (Ok(_res1), Ok(_res2)) => {
                                //         let new_pos = Position::new([p.pair[0].to_string(), p.pair[1].to_string()], *p.crypto_1.last().unwrap(), *p.crypto_2.last().unwrap(), p.zscore, size[0], size[1]);
                                //         positions.insert(p.pair_id.to_string(), new_pos);
                                //     },
                                //     _ => println!("Error posting order to api")
                                // }
                            },
                            _ => println!("Not enough money!"),
                        }
                    }else{
                        println!("Pair: {}, Zscore: {}|{}: {}, {}: {}",&p.pair_id, &p.zscore, &p.pair[0],&p.crypto_1.last().unwrap(),&p.pair[1],&p.crypto_2.last().unwrap());
                    }
                }
            }   
        }
    }

    println!("Program stopped");
}
