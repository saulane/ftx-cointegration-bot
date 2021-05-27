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
pub struct Market{
    pub name:String,
    enabled:bool,
    postOnly:bool,
    priceIncrement:f64,
    sizeIncrement:f64,
    minProvideSize:f64,
    pub last:f64,
    bid:f64,
    ask:f64,
    price:f64,
    r#type:String,
    baseCurrency:Option<String>,
    quoteCurrency:Option<String>,
    underlying:String,
    restricted:bool,
    highLeverageFeeExempt:bool,
    change1h:f64,
    change24h:f64,
    changeBod:f64,
    quoteVolume24h:f64,
    volumeUsd24h:f64,
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
