#[macro_use] extern crate rocket;
use rocket::serde::json::Json;

use std::env;
use redis::Commands;

use serde_json;
use lib::schema;

#[get("/tip?<symbol>")]
async fn tip( symbol: String) -> Json<schema::DepthStreamData> {
    let redis_url = env::var("REDIS_URL").unwrap_or("redis://127.0.0.1:6379".to_string());

    let mut conn = redis::Client::open(redis_url.to_string()).unwrap()
        .get_connection().unwrap();

    let data: String = conn.get(&symbol).unwrap();
    let data: schema::DepthStreamData = serde_json::from_str(&data).unwrap();
    Json(data)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/book", routes![tip])
}
