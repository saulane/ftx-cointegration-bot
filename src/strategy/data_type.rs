use serde_json::{Value};
use serde::{Deserialize, Serialize};

#[path = "calculations.rs"]
mod calculations;

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
    high: f64,
    low: f64,
    pub close: f64,
    volume: f64
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
    pub crypto_1: Vec<f64>,
    pub crypto_2: Vec<f64>,
    pub last_bar_ts: f64,
    pub zscore: f64,
}


impl Pair{
    pub fn new(crypto_1_symbol:String, crypto1: &HistoricalData, crypto_2_symbol: String, crypto2: &HistoricalData) -> Pair{
        let prices_1 = crypto1.prices().unwrap();
        let prices_2 = crypto2.prices().unwrap();

        let log_diff = calculations::log_diff(&prices_1, &prices_2).unwrap();

        Pair{
            pair: [crypto_1_symbol, crypto_2_symbol],
            crypto_1: crypto1.prices().unwrap(),
            crypto_2: crypto2.prices().unwrap(),
            last_bar_ts: crypto1.result.last().unwrap().time,
            zscore: calculations::zscore(&log_diff).unwrap(),
        }
    }

    pub fn update_zscore(&mut self){
        let log_diff = calculations::log_diff(&self.crypto_1, &self.crypto_2).unwrap();

        self.zscore = calculations::zscore(&log_diff).unwrap();
    }

    pub fn update_prices(&mut self, crypto_1: &HistoricalData, crypto_2: &HistoricalData){
        self.crypto_1 = crypto_1.prices().unwrap();
        self.crypto_2 = crypto_2.prices().unwrap();
        self.last_bar_ts = crypto_1.result.last().unwrap().time;

        self.update_zscore()
    }

    pub fn position_size(&self, freeBalance: &f64, totalBalance: &f64) -> Option<[f64;2]>{
        match self.zscore{
                zscore if zscore.abs()>1.5 => {
                    let crypto_1_price: &f64 = self.crypto_1.last().unwrap();
                    let crypto_2_price: &f64 = self.crypto_2.last().unwrap();
            
                    let total_with_leverage: f64 = totalBalance*17.0;
                    let free_with_leverage: f64 = freeBalance*17.0;
            
                    let each_pos_size: f64 = 0.1 * &free_with_leverage;
            
                    let crypto_1_size: f64 = &each_pos_size/crypto_1_price;
                    let crypto_2_size: f64 = &each_pos_size/crypto_2_price;
            
                    match (free_with_leverage, total_with_leverage){
                        (free, total) if free >= 0.2*total=> {
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