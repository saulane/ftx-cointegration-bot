use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::calculations;


#[derive(Serialize, Deserialize, Debug)]
pub struct DataFile{
    pub last_update: u128,
    pub trades: Vec<Trade>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade{
    pub pair: String,
    pub profit: f64,
    pub time: u128
}


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
    pub max_pos: u32,
}


impl Pair{
    pub fn new(crypto_1_symbol:String, crypto1: &HistoricalData, crypto_2_symbol: String, crypto2: &HistoricalData, max_pos:u32) -> Result<Pair,Box<dyn Error>>{
        let prices_1 = crypto1.prices().unwrap();
        let prices_2 = crypto2.prices().unwrap();

        let pair_id = format!("{}/{}", &crypto_1_symbol, &crypto_2_symbol);

        let log_diff = calculations::log_diff(&prices_1, &prices_2)?;
        Ok(Pair{
            pair: [crypto_1_symbol, crypto_2_symbol],
            pair_id,
            crypto_1: crypto1.prices().unwrap(),
            crypto_2: crypto2.prices().unwrap(),
            last_bar_ts: crypto1.result.last().unwrap().time,
            zscore: calculations::zscore(&log_diff)?,
            max_pos
        })
    }

    pub fn update_zscore(&mut self) -> Result<(), Box<dyn Error>>{
        match calculations::log_diff(&self.crypto_1, &self.crypto_2){
            Ok(log_diff) => {
                
                match calculations::zscore(&log_diff){
                    Ok(zs) => {
                        self.zscore = zs;
                        return Ok(());
                    },
                    Err(e) => return Err(e),
                }
            },
            Err(e)=> return Err(e),
        }
    }

    pub fn update_prices(&mut self, crypto_1: &HistoricalData, crypto_2: &HistoricalData) -> Result<(), ()>{
        let _old_crypto1 = std::mem::replace(&mut self.crypto_1, crypto_1.prices().unwrap());
        let _old_crypto2 = std::mem::replace(&mut self.crypto_2, crypto_2.prices().unwrap());
        // self.crypto_1 = crypto_1.prices().unwrap();
        // self.crypto_2 = crypto_2.prices().unwrap();
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

    pub fn position_size(&self, free_balance: &f64, total_balance: &f64) -> Option<[f64;2]>{
        match self.zscore{
                zscore if zscore.abs()>1.5 => {
                    let crypto_1_price: &f64 = self.crypto_1.last().unwrap();
                    let crypto_2_price: &f64 = self.crypto_2.last().unwrap();
                    
                    let c1_len:f64 = (10.0 as f64).powi(10 * calculations::number_of_tens(crypto_1_price) as i32);
                    let c2_len:f64 = (10.0 as f64).powi(10 * calculations::number_of_tens(crypto_2_price) as i32);

                    let total_with_leverage: f64 = total_balance*5.0;
                    let free_with_leverage: f64 = free_balance*5.0;
            
                    let each_pos_size: f64 = (0.9/2.0/self.max_pos as f64)* &total_with_leverage;
            
                    let crypto_1_size: f64 = ((&each_pos_size/crypto_1_price)*c1_len).floor()/c1_len;
                    let crypto_2_size: f64 = ((&each_pos_size/crypto_2_price)*c2_len).floor()/c2_len;
            
                    match (free_with_leverage, total_with_leverage){
                        (free, total) if free >= 2.0*each_pos_size=> {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance{
    pub success: bool,
    pub result: Vec<BalanceCoin>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceCoin{
    pub coin: String,
    pub total: f64,
    pub free: f64,
    pub availableWithoutBorrow: f64,
    pub usdValue: f64,
    pub spotBorrow: f64,
}

impl Balance{
    pub fn get_usd_Balance(&self) -> [f64; 2]{
        let mut balance:[f64;2] = [0.0;2];

        for i in &self.result{
            if i.coin == "USD"{
                balance[0] = i.free;
                balance[1] = i.total;
            }
        }

        balance
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OrderResponse{
    pub success: bool,
    pub result: OrderResult
}
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderResult{
    pub createdAt: String,
    pub filledSize: f64,
    pub future: String,
    pub id: u64,
    pub market: String,
    pub price: f64,
    pub remainingSize: u64,
    pub side: String,
    pub size: f64,
    pub status: String,
    pub r#type: String,
    pub reduceOnly: bool,
    pub ioc: bool,
    pub postOnly: bool,
    pub clientId: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Markets{
    pub success:bool,
    pub result: Vec<Market>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Market{
    pub name:Option<String>,
    enabled:bool,
    postOnly:bool,
    priceIncrement:f64,
    sizeIncrement:f64,
    pub last:f64,
    bid:f64,
    ask:f64,
    price:f64,
    pub r#type:Option<String>,
    baseCurrency: Option<String>,
    quoteCurrency: Option<String>,
    underlying: Option<String>,
    restricted:bool,
    highLeverageFeeExempt:bool,
    change1h:f64,
    change24h:f64,
    changeBod:f64,
    quoteVolume24h:f64,
    pub volumeUsd24h:f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenPosition{
    pub cost:String,
    pub entryPrice:f64,
    pub estimatedLiquidationPrice: f64,
    pub future: String,
    pub initialMarginRequirement: f64,
    pub longOrderSize: f64,
    pub maintenanceMarginRequirement: f64,
    pub netSize: f64,
    pub openSize: f64,
    pub realizedPnl: f64,
    pub shortOrderSize: f64,
    pub side: String,
    pub size: f64,
    pub unrealizedPnl: f64,
    pub collateralUsed: f64,
}