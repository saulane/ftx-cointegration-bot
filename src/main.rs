extern crate reqwest;

mod auth;
mod ws;
mod sign;

const API_KEY: &str = "Lo5C3gwBRYav_-hECTlY2ej85iwkegqYEJU24Ip-";
const API_SECRET: &str = "Yvek0WTrOw03QQYbP4B-ZQ2S_FZoNXExtfCGYrAf";

#[tokio::main]
async fn main() {


    println!("{:?}",sign::signature("GET/api/markets").unwrap());

    let api_client = reqwest::Client::new();
    let test = auth::get_jsoned_data(api_client, "BTC-PERP", "300").await.unwrap();
    println!("{:?}", test);



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



