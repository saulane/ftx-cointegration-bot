use std::cell::RefCell;

#[path = "../utils.rs"]
mod utils;

#[derive(Debug, Clone)]
pub struct MarketState{
    pub btc_price: f64,
    pub bch_price: f64,
    pub last_20_mean: RefCell<Vec<f64>>,
    pub last_update_ts: u128,
}

fn mean_dist(btc: f64, bch:f64) -> f64{
    println!("Log BTC: {}, Log BCH: {}", btc.ln(), bch.ln());
    return bch.ln()-btc.ln();
}

impl MarketState{
    pub fn new() -> MarketState{
        MarketState{
            btc_price: 0.0,
            bch_price: 0.0,
            last_20_mean: RefCell::new(Vec::new()),
            last_update_ts: utils::current_ts(),
        }
    }

    pub fn update_price(&mut self, btc: f64, bch: f64){
        self.btc_price = btc;
        self.bch_price = bch;
        self.last_20_mean.borrow_mut().push(mean_dist(btc, bch));
        self.last_update_ts = utils::current_ts();
    }
}