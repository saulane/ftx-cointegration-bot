use std::error::Error;

pub fn mean(data: &Vec<f64>) -> Option<f64>{
    Some(data.iter().sum::<f64>() as f64/ data.len() as f64)
}

pub fn zscore(data: &Vec<f64>) -> Result<f64, Box<dyn Error>>{
    // let data = &self.diff_history.borrow().clone();
    // let mean = mean(data);
    // let std_deviation = std_deviation(data);

    // println!("Mean: {:?}, StdDev: {:?}", mean, std_deviation);

    match (mean(data), std_deviation(data), data.last()){
        (Some(mean), Some(std), Some(last)) => return Ok((last-mean)/std),
        _ => return Err("Error computing zscore.".into()),
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

pub fn log_diff(data1: &Vec<f64>, data2: &Vec<f64>) -> Result<Vec<f64>, Box<dyn Error>>{
    match (data1, data2) {
        (d1, d2) if d1.len() == d2.len() => {
            let data_len: usize = d1.len();
            let mut log_diff = Vec::new();

            for i in 0..data_len{
                log_diff.push(d1[i].ln() - d2[i].ln());
            }

            return Ok(log_diff);

        },
        _ => return Err("Error computing the log difference: wrong size.".into()),
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