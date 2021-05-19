extern crate hex;

use std::time::{SystemTime};
use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};

type HmacSha256 = Hmac<Sha256>;

pub fn current_ts() -> u64 {
    let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    ts
}


pub fn signature(endpoint: &str) -> Result<String, ()> {
    let ts: String = current_ts().to_string();

    let message = format!("{}{}", ts, endpoint);


    let secret = "T4lPid48QtjNxjLUFOcUZghD7CUJ7sTVsfuvQZF2";

    let mut mac = HmacSha256::new_from_slice(&secret.as_bytes()).expect("Problem keying the API_SECRET");
    mac.update(&message.as_bytes());
    let result = mac.finalize().into_bytes();

    let r2 = hex::encode(&result);

    Ok(r2)
}