extern crate hex;

use std::time::{SystemTime};
use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};


pub fn current_ts() -> u128 {
    let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    ts
}


pub fn signature(api_secret: &str, time: u128, endpoint: &str, method: &str) -> Result<String, ()> {
    type HmacSha256 = Hmac<Sha256>;
    let ts: String = time.to_string();

    let message = format!("{}{}{}", ts, method, endpoint);
    println!("Sign Payload: {}", message);

    let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes()).expect("Problem keying the API_SECRET");
    mac.update(&message.as_bytes());
    let result = mac.finalize().into_bytes();

    let r2 = hex::encode(&result);
    
    // println!("{}",message);
    // println!("{}",r2);

    Ok(r2)
}