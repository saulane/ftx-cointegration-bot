fn slope(x: &Vec<f64>, y: &Vec<f64>) -> f64{
    let n = x.len() as f64;

    let mut sumxy:f64 = 0.0;
    for i in 0..x.len(){
        sumxy+= x[i]*y[i]
    }

    let num:f64 = n * sumxy - x.iter().sum::<f64>() * y.iter().sum::<f64>();
    let denom:f64 = n * x.iter().map(|&i| (i.powi(2))).sum::<f64>() - x.iter().sum::<f64>().powi(2);

    return num/denom
}

fn se(x: &Vec<f64>, y: &Vec<f64>, slope: &f64) -> f64{
    //let slope = slope(x,y);
    let mut olsresiduals: Vec<f64> = Vec::new();
    for i in 0..x.len(){
        olsresiduals.push( y[i] - slope * x[i] );
    }

    let x_residuals: Vec<f64> = x[1..].to_vec();
    let mean_x_residuals = mean(&x_residuals);
    let n = olsresiduals.len();

    // let num = ( residuals.iter().map( |&x1| ( x1.powi(2) ) ).sum::<f64>() / ( n as f64 - 2.0 ) ).sqrt();
    // let denom = x.iter().map(|&x2| ( (x2 - mean_x_residuals).powi(2) )).sum::<f64>().sqrt() ;

    let num = ( olsresiduals.iter().map(|&i| (i.powi(2))).sum::<f64>()  / (n-2) as f64).sqrt();
    let denom = ( x_residuals.iter().map(|&i| ( (i-mean_x_residuals).powi(2) )).sum::<f64>() ).sqrt();
    // println!("SE: {}", num/denom);

    num/denom
}

fn mean(x: &Vec<f64>) -> f64{
    x.iter().sum::<f64>() as f64/ x.len() as f64
}

pub fn tstat(x: &Vec<f64>, y: &Vec<f64>, slope: &f64) -> f64{
    let tstat = slope/se(x, y, slope);
    // println!("{}", tstat);
    tstat
    
}


pub fn coint(x: &Vec<f64>, y: &Vec<f64>) -> Result<bool, Box<dyn std::error::Error>>{
    if x.len()!= y.len(){
        return Err("Datas are not the same size!".into());
    }

    let critical_value: f64 = -3.4369259442540416;

    let slope_1 = slope(x,y);
    // println!("COEF X1: {:?}", &slope_1);

    //Difference between Y and the predicted value of the linear regression Z = Y - aX  
    let mut z:Vec<f64> = Vec::new();
    for i in 0..x.len(){
        z.push(y[i] - slope_1 * x[i]);
    }


    let mut delta_resid:Vec<f64>  = Vec::new();
    for i in 1..z.len(){
        delta_resid.push( z[i] - z[i-1] );
    }

    let t1residuals = &z[..z.len()-1].to_vec();
    
    let slope_2 = slope(&t1residuals, &delta_resid);

    // println!("COEF t1 {:?}", &slope_2);
    match tstat(&t1residuals, &delta_resid, &slope_2){
        val if val <= critical_value => Ok(true),
        _ => Ok(false)
    }
}