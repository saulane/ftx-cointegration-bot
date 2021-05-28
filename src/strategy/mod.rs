pub mod calculations;
pub mod data_type;
pub mod python_script;

use std::{cell::RefCell};

#[derive(Debug, Clone)]
pub struct MarketState{
    pub btc_price: f64,
    pub bch_price: f64,
    pub diff_history: RefCell<Vec<f64>>,
    pub last_update_ts: u128,
    pub rolling_20_mean: Option<f64>,
    pub zscore: Option<f64>,
}
