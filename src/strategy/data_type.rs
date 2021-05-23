use serde_json::{Value};
use serde::{Deserialize, Serialize};

#[path = "calculations.rs"]
mod calculations;

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoricalData{
    success: bool,
    result: [PriceData; 20],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceData{
    startTime: String,
    time: f64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64
}

impl HistoricalData{
    pub fn new(crypto: Value) -> Result<HistoricalData, Box<dyn std::error::Error>>{
        let historical_data: HistoricalData = serde_json::from_value(crypto)?;
        Ok(historical_data)
    }

    pub fn prices(&self) -> Option<[f64;20]>{
        let mut prices:[f64; 20] = [0.0; 20];

        for (i,x) in self.result.iter().enumerate(){
            prices[i] = x.close;
        }

        Some(prices)

    }
}


pub struct Pair{
    pub crypto_1: [f64; 20],
    pub crypto_2: [f64; 20],
    pub zscore: f64,
}


impl Pair{
    pub fn new(crypto1: &HistoricalData, crypto2: &HistoricalData) -> Pair{
        let prices_1: [f64; 20] = crypto1.prices().unwrap();
        let prices_2: [f64; 20] = crypto2.prices().unwrap();

        let log_diff = calculations::log_diff(&prices_1, &prices_2).unwrap();

        Pair{
            crypto_1: crypto1.prices().unwrap(),
            crypto_2: crypto2.prices().unwrap(),
            zscore: calculations::zscore(&log_diff).unwrap(),
        }
    }
}