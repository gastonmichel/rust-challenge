use tungstenite::connect;
use url::Url;
use serde_json;
use redis::Commands;
use std::env;

use lib::schema;

const SUPPORTED_ASSETS: [&str; 2] = ["btcusdt@depth20", "ethusdt@depth20"];

fn main() {    
    
    let redis_url = env::var("REDIS_URL").unwrap_or("redis://127.0.0.1:6379".to_string());
    
    let mut conn = redis::Client::open(redis_url.to_string())
        .expect("Could not create redis client")
        .get_connection()
        .expect("could not create connection");

    println!("Connected to redis.");

    let binance_ws_url = env::var("BINANCE_WEBSOCKET_URL").unwrap_or("wss://stream.binance.com:9443".to_string());
    let binance_url = format!(
        "{url}/stream?streams={streams}", 
        url=binance_ws_url, 
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