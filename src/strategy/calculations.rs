use std::{borrow::Borrow, cell::RefCell};

#[path = "../utils.rs"]
mod utils;

#[derive(Debug, Clone)]
pub struct MarketState{
    pub btc_price: f64,
    pub bch_price: f64,
    pub diff_history: RefCell<Vec<f64>>,
    pub last_update_ts: u128,
    pub rolling_20_mean: Option<f64>,
    pub zscore: Option<f64>,
}

fn mean_dist(btc: f64, bch:f64) -> f64{
    println!("Log BTC: {}, Log BCH: {}", btc.ln(), bch.ln());
    return bch.ln()-btc.ln();
}


fn mean(data: &Vec<f64>) -> Option<f64>{
    // let data = &self.diff_history.borrow().clone();
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();

    match count{
        notempty if notempty>=20 => Some(sum/count as f64),
        _ => None,
    }
}

fn zscore(data: &Vec<f64>) -> Option<f64>{
    // let data = &self.diff_history.borrow().clone();
    let mean = mean(data);
    let std_deviation = std_deviation(data);

    let zscore = match (mean, std_deviation) {
        (Some(mean), Some(std_deviation)) => {
            let diff = data[4] as f64 - mean;

            Some(diff / std_deviation)
        },
        _ => None
    };

    zscore
}

fn std_deviation(data: &Vec<f64>) -> Option<f64> {
    // let data = &self.diff_history.borrow().clone();
    match (mean(data), data.len()) {
        (Some(mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = mean - (*value as f64);

                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    }

}

impl MarketState{
    pub fn new() -> MarketState{
        MarketState{
            btc_price: 0.0,
            bch_price: 0.0,
            diff_history: RefCell::new(Vec::new()),
            last_update_ts: utils::current_ts(),
            rolling_20_mean: None,
            zscore: None,
        }
    }

    pub fn update_price(&mut self, btc: f64, bch: f64) -> Option<f64>{
        let mut data = self.diff_history.borrow_mut();
        self.btc_price = btc;
        self.bch_price = bch;
        self.last_update_ts = utils::current_ts();

        match data.len(){
            long_enough if long_enough>20 => {
                data.push(mean_dist(btc, bch));
                data.remove(0);
                
                self.rolling_20_mean = mean(&data);
                self.zscore = zscore(&data);

                Some(zscore(&data).unwrap())
            },
            _ => {
                data.push(mean_dist(btc, bch));
                None
            },
        }
    }

    fn filter_history(&mut self) -> f64{
        self.diff_history.borrow_mut().remove(0);
        let mut sum:f64 = 0.0;
        let mut size:f64 = 0.0;
        for x in self.diff_history.borrow_mut().iter(){
            sum = sum + x;
            size = size+1.0;
        }
        self.rolling_20_mean = Some(sum/size);
        sum / size
    }

    fn last_diff(self) -> Option<f64>{
        self.diff_history.borrow().last().copied()
    }

}