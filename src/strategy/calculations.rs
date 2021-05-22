pub fn mean_dist(btc: f64, bch:f64) -> f64{
    println!("Log BTC: {}, Log BCH: {}", btc.ln(), bch.ln());
    return bch.ln()-btc.ln();
}


pub fn mean(data: &Vec<f64>) -> Option<f64>{
    // let data = &self.diff_history.borrow().clone();
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();

    match count{
        notempty if notempty>=20 => Some(sum/count as f64),
        _ => None,
    }
}

pub fn zscore(data: &Vec<f64>) -> Option<f64>{
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

