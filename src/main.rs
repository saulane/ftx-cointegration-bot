extern crate reqwest;
#[macro_use]
extern crate lazy_static;

mod modules;

use std::collections::HashMap;
use modules::rest_api::FtxApiClient;
use modules::data_type::{HistoricalData, Pair, Position};
use modules::utils;

use serde_json::Value;

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

#[tokio::main]
async fn main(){
    let ftx_bot =  FtxApiClient{
        api_key: API_KEY.to_string(),
        api_secret: API_SECRET.to_string(),
        request_client: reqwest::Client::new()
    };

    println!("Positions Ouvertes: {:?}", ftx_bot.get_open_positions().await.unwrap().into_iter().filter(|i| i["future"] == "ETH-PERP").collect::<Vec<Value>>()  );

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

    println!("Dernière mise à jour des paires: {}", utils::read_last_pairs_update());

    let mut pairs_coint_test = utils::coint_pairs_list(&ftx_bot).await.unwrap();

    let mut pair_list: Vec<Pair> = utils::make_pair_list(&pairs_coint_test.0, &ftx_bot).await.unwrap();
    let mut positions: HashMap<String, Position> = HashMap::new();


    //Main Infinite Loop
    'mainloop: loop {
        if utils::current_ts() - utils::read_last_pairs_update() >= 86400000{
            println!("Updating Cointegrated Pairs List");
            pairs_coint_test = utils::coint_pairs_list(&ftx_bot).await.unwrap();
            pair_list = utils::make_pair_list(&pairs_coint_test.0, &ftx_bot).await.unwrap();
        }

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
            
            //Check if the Pair is already in position
            if positions.contains_key(&p.pair_id){
                unsafe{
                    if p.zscore.abs() <= 0.5 || STOP == true{            
                        println!("Closing Trade on pair: {}, zscore={}", &p.pair_id, &p.zscore);
                        let curr_crypto1_pos_size = -&positions[&p.pair_id].crypto1_size;
                        let curr_crypto2_pos_size = -&positions[&p.pair_id].crypto2_size;

                        let crypto1_profit = if &positions[&p.pair_id].crypto1_size >= &0.0 { p.crypto_1.last().unwrap()/&positions[&p.pair_id].crypto1_entry_price - 1.0 } else { &positions[&p.pair_id].crypto1_entry_price/p.crypto_1.last().unwrap() - 1.0 };
                        let crypto2_profit = if &positions[&p.pair_id].crypto2_size >= &0.0 { p.crypto_2.last().unwrap()/&positions[&p.pair_id].crypto2_entry_price - 1.0 } else { &positions[&p.pair_id].crypto2_entry_price/p.crypto_2.last().unwrap() - 1.0 };
                        
                        utils::save_trade(&p.pair_id, (&crypto1_profit+&crypto2_profit-0.14/100.0)*100.0);
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
                        println!("Open Trade on {}, zs={}, | {} x {}, {} x {}", &p.pair_id, &p.zscore, &p.crypto_1.last().unwrap(), &positions.get(&p.pair_id).unwrap().crypto1_size,&p.crypto_2.last().unwrap(), &positions.get(&p.pair_id).unwrap().crypto2_size );
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
                            _ => println!("Not enough money on balance!"),
                        }
                    }else{
                        // println!("Pair: {}, Zscore: {}|{}: {}, {}: {}",&p.pair_id, &p.zscore, &p.pair[0],&p.crypto_1.last().unwrap(),&p.pair[1],&p.crypto_2.last().unwrap());
                    }
                }
            }
            //Waiting between each pairs to not make more than 30 requests/sec
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        //Waiting in order to wait for the API to update OHLC
        std::thread::sleep(std::time::Duration::from_secs(10));
    }

    println!("Program stopped");
}
