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
    pub crypto_1: Vec<f64>,
    pub crypto_2: Vec<f64>,
    pub last_bar_ts: f64,
    pub zscore: f64,
}


impl Pair{
    pub fn new(crypto1: &HistoricalData, crypto2: &HistoricalData) -> Pair{
        let prices_1 = crypto1.prices().unwrap();
        let prices_2 = crypto2.prices().unwrap();

        let log_diff = calculations::log_diff(&prices_1, &prices_2).unwrap();

        Pair{
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

    pub fn decision_making(&self) -> Result<bool, ()>{
        match &self.zscore{
            zscore if zscore.abs() > 1.5 => Ok(true),
            _ => Ok(false),
        }
    }
}