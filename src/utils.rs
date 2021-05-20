extern crate hex;

use std::time::{SystemTime};
use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};


pub fn current_ts() -> u128 {
    let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    ts
}


pub fn signature(api_secret: &String, endpoint: &str, method: &str) -> Result<(String, String), ()> {
    type HmacSha256 = Hmac<Sha256>;
    let ts: String = current_ts().to_string();

    let message = format!("{}{}/api{}", ts, method, endpoint);

    let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes()).expect("Problem keying the API_SECRET");
    mac.update(&message.as_bytes());
    let result = mac.finalize().into_bytes();

    let r2 = hex::encode(&result);

    Ok((r2, ts))
}