extern crate reqwest;
#[macro_use]
extern crate lazy_static;

mod rest_api;
mod ws;
mod utils;

use rest_api::FtxApiClient;

lazy_static!{
    static ref API_KEY: String = std::env::var("FTX_API_KEY").unwrap();
    static ref API_SECRET: String = std::env::var("FTX_API_SECRET").unwrap();
}

#[tokio::main]
async fn main() {
    
    let ftx_bot =  FtxApiClient{
        api_key: API_KEY.to_string(),
        api_secret: API_SECRET.to_string(),
        request_client: reqwest::Client::new()
    };


    let test = ftx_bot.fetch_historical_data("BTC-PERP", "300").await.unwrap();
    println!("{:?}", test);

    let test_balance = ftx_bot.get_balance().await.unwrap();
    println!("{:?}", test_balance);


    // let (mut socket, response) =
    //     connect(Url::parse("wss://ftx.com/ws/").unwrap()).expect("Can't connect");

    // println!("Connected to the server");
    // println!("Response HTTP code: {}", response.status());

    // let subcribe_msg  = ws::subscribe("ticker");

    // socket.write_message(Message::Text(subcribe_msg.to_string())).unwrap();
    // loop {
    //     let msg = socket.read_message().expect("Error reading message");
    //     let msg = match msg {
    //         tungstenite::Message::Text(s) => { s }
    //         _ => { panic!() }
    //     };
    //     let parsed: Value = serde_json::from_str(&msg).expect("Can't parse to JSON");

    //     println!("Last price: {:?}$", parsed["data"]["last"]);
    // }
    // socket.close(None);
}



