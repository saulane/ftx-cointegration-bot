use serde::{Deserialize, Serialize};

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
    pub fn get_USD_Balance(&self) -> [f64; 2]{
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