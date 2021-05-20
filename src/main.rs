extern crate reqwest;

mod auth;
mod ws;
mod sign;

const API_KEY: &str = "Wij-a7W4DTm0Ly_5vJqu9DZrhRdzG_ma4OuAeMAP";
const API_SECRET: &str = "JtPh-moNuFE5eyRvqA3fFfBqMPEFk7ix3B-mc5-E";

#[tokio::main]
async fn main() {


    //println!("{:?}",sign::signature("T4lPid48QtjNxjLUFOcUZghD7CUJ7sTVsfuvQZF2","/api/markets", 1588591511721, "GET").unwrap());

    let api_client = reqwest::Client::new();

    // let test = auth::get_jsoned_data(api_client, "BTC-PERP", "300").await.unwrap();
    // println!("{:?}", test);

    let test_balance = auth::get_balance(api_client, API_KEY, API_SECRET).await.unwrap();
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



