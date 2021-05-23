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

pub fn std_deviation(data: &Vec<f64>) -> Option<f64> {
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

fn mean_array(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn std_deviation_array(data: &[f64]) -> Option<f64> {
    match (mean_array(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f64);

                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    }
}

pub fn zscore_array(data: &[f64]) -> Option<f64>{
    // let data = &self.diff_history.borrow().clone();
    let mean = mean_array(data);
    let std_deviation = std_deviation_array(data);

    let zscore = match (mean, std_deviation) {
        (Some(mean), Some(std_deviation)) => {
            let diff = data[4] as f64 - mean;

            Some(diff / std_deviation)
        },
        _ => None
    };

    zscore
}


pub fn log_diff<'a>(data1: &'a [f64], data2: &'a [f64]) -> Option<Vec<f64>>{
    if data1.len() != data2.len(){
        return None
    }else{
        let data_len: usize = data1.len();
        let mut log_diff = Vec::new();

        for i in 0..data_len+1{
            log_diff.push(data1[i].ln() - data2[i].ln());
        }

        return Some(log_diff);
    }
}