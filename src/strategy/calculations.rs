struct MarketState{
    btc_price: f64,
    bch_price: f64,
    log_btc: f64,
    log_bch: f64,
    last_20_mean: Vec,
    zscore: f64,
};

pub fn init_market_state(btc: f64, bch:f64) -> MarketState{
    MarketState{
        btc_price: btc,
        bch_price: bch,
        log_btc: btc.log(),
        log_bch: bch.log(),
        last_20_mean: vec![mean_dist(log_btc, log_bch)],
    }   
};

fn mean_dist(lbtc: f64, lbch:f64) -> f64{
    lbch-lbtc
}

impl MarketState{
    pub fn update_price(&self, btc: f64, bch: f64){
        &self.btc_price = btc;
        &self.bch_price = bch;
    }

    fn log_calculation(&self){
        &self.log_btc = 
    }
    
};