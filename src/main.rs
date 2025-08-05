use tungstenite::connect;
use url::Url;
use serde_json;
use std::env;
use std::thread;
// use std::time::Duration;
use std::sync::mpsc;

mod schema;
// mod repository;

const SUPPORTED_ASSETS: [&str; 2] = ["btcusdt@depth20", "ethusdt@depth20"];

fn main() {    
    let (tx, rx) = mpsc::channel();
    
    let handle = thread::spawn( move || {
        listen_order_book_updates(tx)
    });

    for received in rx {
        println!("Got: {received:?}");
    }

    handle.join().unwrap();
}



fn listen_order_book_updates(tx: mpsc::Sender<schema::DepthStreamData>) {
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
        println!("{:?}",msg);
        let _msg: schema::DepthStreamMessage = match msg {
            tungstenite::Message::Text(s) => serde_json::from_str(s.as_str()).unwrap(),
            tungstenite::Message::Ping(_) => continue,
            _ => break
        };
        tx.send(_msg.data);

    }
}