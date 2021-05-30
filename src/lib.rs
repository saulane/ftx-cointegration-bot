pub mod utils{
    pub extern crate hex;
    pub use std::time::{SystemTime};
    pub use sha2::Sha256;
    pub use hmac::{Hmac, Mac, NewMac};
    use csv::StringRecord;
    use std::fs;
    use std::error::Error;

    pub fn current_ts() -> u128 {
        let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        ts
    }

    pub fn signature(api_secret: &str, endpoint: &str, method: &str, api: &str) -> Result<(String, String), ()> {
        type HmacSha256 = Hmac<Sha256>;
        let ts: String = current_ts().to_string();

        let message = if api=="rest" {format!("{}{}/api{}", ts, method, endpoint)} else {format!("{}websocket_login", ts)};
        //println!("URL payload: {}", message);

        let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes()).expect("Problem keying the API_SECRET");
        mac.update(&message.as_bytes());
        let result = mac.finalize().into_bytes();

        let r2 = hex::encode(&result);

        Ok((r2, ts))
    }

    pub fn pairs_reader() -> Result<Vec<StringRecord>, Box<dyn Error>> {
        let file = fs::File::open("CointPairs.txt").expect("Pls copy your 'CointPairs.txt' in the root folder of this app");
        let mut rdr = csv::Reader::from_reader(file);
        let mut pairs_vec = Vec::new();
        for result in rdr.records() {
            let record = result?;
            pairs_vec.push(record);
        }
        Ok(pairs_vec)
    }
}