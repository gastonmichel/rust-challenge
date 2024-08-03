use tungstenite::connect;
use url::Url;
use serde_json;
use redis::Commands;

use lib::schema;

const  BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

const SUPPORTED_ASSETS: [&str; 2] = ["btcusdt@depth20", "ethusdt@depth20"];

fn main() {    
    println!("Connected to redis.");

    let mut conn = redis::Client::open("redis://127.0.0.1/")
        .expect("Could not create redis client")
        .get_connection()
        .expect("could not create connection");

    let binance_url = format!(
        "{url}/stream?streams={streams}", 
        url=BINANCE_WS_API, 
        streams=SUPPORTED_ASSETS.join("/")
    );

    println!("{}",binance_url);

    let (mut socket, _) =
        connect(Url::parse(&binance_url).unwrap().as_str()).unwrap();

    println!("Connected to binance stream.");

    loop {
        let msg = socket.read().expect("Error reading message");
        let msg: schema::DepthStreamMessage = match msg {
            tungstenite::Message::Text(s) => serde_json::from_str(s.as_str()).unwrap(),
            tungstenite::Message::Ping(_) => continue,
            _ => break
        };

        let _: () = conn.set(msg.stream.to_string(), serde_json::to_string(&msg.data).unwrap()).unwrap();
        println!("{}",msg.stream)
    }
}