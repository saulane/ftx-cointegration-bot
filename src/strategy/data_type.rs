use std::fmt::format;

use serde_json::{Value};
use serde::{Deserialize, Serialize};

#[path = "calculations.rs"]
mod calculations;

use calculations::ZscoreError;

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoricalData{
    pub success: bool,
    pub result: Vec<PriceData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceData{
    pub startTime: String,
    pub time: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64
}

impl HistoricalData{
    pub fn new(crypto: Value) -> Result<HistoricalData, Box<dyn std::error::Error>>{
        let historical_data: HistoricalData = serde_json::from_value(crypto)?;
        Ok(historical_data)
    }

    pub fn prices(&self) -> Option<Vec<f64>>{
        let mut prices: Vec<f64> = Vec::new();

        for i in self.result.iter(){
            prices.push(i.close);
        }

        Some(prices)

    }
}

#[derive(Debug)]
pub struct Pair{
    pub pair: [String; 2],
    pub pair_id: String,
    pub crypto_1: Vec<f64>,
    pub crypto_2: Vec<f64>,
    pub last_bar_ts: f64,
    pub zscore: f64,
}


impl Pair{
    pub fn new(crypto_1_symbol:String, crypto1: &HistoricalData, crypto_2_symbol: String, crypto2: &HistoricalData) -> Result<Pair, ZscoreError>{
        let prices_1 = crypto1.prices().unwrap();
        let prices_2 = crypto2.prices().unwrap();

        let pair_id = format!("{}/{}", &crypto_1_symbol, &crypto_2_symbol);

        let log_diff = match calculations::log_diff(&prices_1, &prices_2){
            Ok(logvec) => Ok(logvec),
            _ => Err(ZscoreError),
        };


        

        Ok(Pair{
            pair: [crypto_1_symbol, crypto_2_symbol],
            pair_id: pair_id,
            crypto_1: crypto1.prices().unwrap(),
            crypto_2: crypto2.prices().unwrap(),
            last_bar_ts: crypto1.result.last().unwrap().time,
            zscore: calculations::zscore(&log_diff?)?,
        })
    }

    pub fn update_zscore(&mut self) -> Result<(), ZscoreError>{
        match calculations::log_diff(&self.crypto_1, &self.crypto_2){
            Ok(log_diff) => {
                
                match calculations::zscore(&log_diff){
                    Ok(zs) => {
                        self.zscore = zs;
                        return Ok(());
                    },
                    _ => return Err(ZscoreError),
                }
            },
            _ => return Err(ZscoreError),
        }
    }

    pub fn update_prices(&mut self, crypto_1: &HistoricalData, crypto_2: &HistoricalData) -> Result<(), ()>{
        self.crypto_1 = crypto_1.prices().unwrap();
        self.crypto_2 = crypto_2.prices().unwrap();
        self.last_bar_ts = crypto_1.result.last().unwrap().time;

        match self.update_zscore(){
            Ok(()) => Ok(()),
            _ => Err(())
        }
    }

    pub fn update_last_prices(&mut self ,crypto1_last: f64, crypto2_last: f64) -> Result<(), ()>{
        std::mem::replace(self.crypto_1.last_mut().unwrap(), crypto1_last);
        std::mem::replace(self.crypto_2.last_mut().unwrap(), crypto2_last);

        match self.update_zscore(){
            Ok(()) => Ok(()),
            _ => Err(())
        }
    }

    pub fn position_size(&self, freeBalance: &f64, totalBalance: &f64) -> Option<[f64;2]>{
        match self.zscore{
                zscore if zscore.abs()>1.5 => {
                    let crypto_1_price: &f64 = self.crypto_1.last().unwrap();
                    let crypto_2_price: &f64 = self.crypto_2.last().unwrap();
            
                    let total_with_leverage: f64 = totalBalance*18.0;
                    let free_with_leverage: f64 = freeBalance*18.0;
            
                    let each_pos_size: f64 = 0.15 * &total_with_leverage;
            
                    let crypto_1_size: f64 = ((&each_pos_size/crypto_1_price)*10000.0).floor()/10000.0;
                    let crypto_2_size: f64 = ((&each_pos_size/crypto_2_price)*10000.0).floor()/10000.0;
            
                    match (free_with_leverage, total_with_leverage){
                        (free, total) if free >= 0.45*total=> {
                            match self.zscore{
                                zs if zs<0.0 => Some([crypto_1_size, -crypto_2_size]),
                                zs if zs>=0.0 => Some([-crypto_1_size, crypto_2_size]),
                                _ => None,
                            }
                        },
                        _ => None
                    }
                }
                _=> None

        }

    }
}


#[derive(Debug)]
pub struct Position{
    pub is_active: bool,
    pub pair: [String; 2],
    pub crypto1_entry_price: f64,
    pub crypto2_entry_price: f64,
    pub zscore: f64,
    pub crypto1_size: f64,
    pub crypto2_size: f64,
}

impl Position{
    pub fn new(pair:[String;2], crypto1_entry_price: f64, crypto2_entry_price: f64, zscore: f64, crypto1_size:f64,crypto2_size:f64) -> Position{
        Position{
            is_active: true,
            pair,
            crypto1_entry_price,
            crypto2_entry_price,
            zscore,
            crypto1_size,
            crypto2_size,
        }
    }

    pub fn update(&mut self, zscore: f64){
        self.zscore = zscore;
    }
}