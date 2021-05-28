use std::fmt::write;


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

pub fn zscore(data: &Vec<f64>) -> Result<f64, ZscoreError>{
    // let data = &self.diff_history.borrow().clone();
    // let mean = mean(data);
    // let std_deviation = std_deviation(data);

    // println!("Mean: {:?}, StdDev: {:?}", mean, std_deviation);

    match (mean(data), std_deviation(data), data.last()){
        (Some(mean), Some(std), Some(last)) => return Ok((last-mean)/std),
        _ => return Err(ZscoreError),
    }
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

pub fn log_diff(data1: &Vec<f64>, data2: &Vec<f64>) -> Result<Vec<f64>, ()>{
    match (data1, data2) {
        (d1, d2) if d1.len() == d2.len() => {
            let data_len: usize = d1.len();
            let mut log_diff = Vec::new();

            for i in 0..data_len{
                log_diff.push(d1[i].ln() - d2[i].ln());
            }

            return Ok(log_diff);

        },
        _ => return Err(()),
    }

}

pub fn number_of_tens(x:&f64) -> u32{
    let mut x_clone = x.clone();
    let mut len = 1;
    while x_clone > 1.0{
        x_clone/=10.0;
        len+=1;
    }
    len
}


#[derive(Debug, Clone)]
pub struct ZscoreError;

impl std::fmt::Display for ZscoreError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "Error computing zscore")
    }
}


#[derive(Debug, Clone)]
pub struct LogDiffError;

impl std::fmt::Display for LogDiffError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "Error computing the log diff of the 2 vec, make sure both Vec are the same size")
    }
}
