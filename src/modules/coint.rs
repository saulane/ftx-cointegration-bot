use std::error::Error;
use super::calculations::mean;

struct LinearRegression{
    x: Vec<f64>,
    y: Vec<f64>,
    n: usize,
}

impl LinearRegression{
    fn new(x: Vec<f64>, y: Vec<f64>, n: usize) -> Result<LinearRegression, Box<dyn Error>>{
        match (x,y,n) {
            (x,y,n) if x.len() == y.len() && y.len() == n => 
                Ok(
                    LinearRegression{
                        x,
                        y,
                        n
                    }
                ),
            _  => Err("X and Y are not the same size".into())
        }
    }

    fn slope(&self) -> f64{
        let mut sumxy:f64 = 0.0;
        for i in 0..self.n{
            sumxy+= self.x[i]*self.y[i]
        }
    
        let num:f64 = self.n as f64 * sumxy - self.x.iter().sum::<f64>() * self.y.iter().sum::<f64>();
        let denom:f64 = self.n as f64 * self.x.iter().map(|&i| (i.powi(2))).sum::<f64>() - self.x.iter().sum::<f64>().powi(2);
    
        return num/denom
    }

    fn residuals(&self) -> Vec<f64>{
        let mut z:Vec<f64> = Vec::new();
        let slope = self.slope();
        for i in 0..self.n{
            z.push(self.y[i] - slope * self.x[i]);
        }
        z
    }

    fn se(&self) -> f64{
        let mut olsresiduals: Vec<f64> = self.residuals();
        // for i in 0..self.n{
        //     olsresiduals.push( self.y[i] - self.slope() * self.x[i] );
        // }
    
        let x_residuals: Vec<f64> = self.x[1..].to_vec();
        let mean_x_residuals = mean(&x_residuals).unwrap();
        let n2 = olsresiduals.len();
    
        let num = ( olsresiduals.iter().map(|&i| (i.powi(2))).sum::<f64>()  / (n2-2) as f64).sqrt();
        let denom = ( x_residuals.iter().map(|&i| ( (i-mean_x_residuals).powi(2) )).sum::<f64>() ).sqrt();
    
        num/denom
    }

    pub fn tstat(&self) -> f64{
        let tstat = self.slope()/self.se();
        tstat
    }
}

pub fn coint(x: &Vec<f64>, y: &Vec<f64>) -> Result<bool, Box<dyn std::error::Error>>{
    if x.len()!= y.len(){
        return Err("Datas are not the same size!".into());
    }

    let critical_value: f64 = -3.4369259442540416;

    let ols_xy: LinearRegression = LinearRegression::new(x.to_vec(),y.to_vec(),x.len())?;
    let ols_xy_residuals = ols_xy.residuals();

    // On soustrait Z[i] par Z[i-1] avec Z = Y - slope*X
    let mut delta_xy_residuals:Vec<f64> = Vec::new();
    for i in 1..ols_xy.n{
        delta_xy_residuals.push(ols_xy_residuals[i] - ols_xy_residuals[i-1])
    }

    // On décale Z de 1 rang
    let t1_xy_residuals:Vec<f64> = ols_xy_residuals[..ols_xy_residuals.len()-1].to_vec();

    // Regression linear entre Z décalé de 1 et Z[i] - Z[i-1]
    let ols_delta_t1: LinearRegression = LinearRegression::new(t1_xy_residuals.to_vec(), delta_xy_residuals, t1_xy_residuals.len())?;
    println!("tstat calculée avec struct: {}", ols_delta_t1.tstat());


    match ols_delta_t1.tstat(){
        val if val <= critical_value => Ok(true),
        _ => Ok(false)
    }
}