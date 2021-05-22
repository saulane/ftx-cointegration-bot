pub mod calculations;

use std::{cell::RefCell};
use calculations::{mean, mean_dist, zscore};

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