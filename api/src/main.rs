#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket_db_pools::{deadpool_redis::{self, redis::AsyncCommands}, Connection, Database};

// use serde::Serialize;
use serde_json;
use lib::schema;

#[derive(Database)]
#[database("redis")]
pub struct RedisPool(deadpool_redis::Pool);

#[get("/tip?<symbol>")]
async fn tip(mut redis: Connection<RedisPool>, symbol: String) -> Json<schema::DepthStreamData> {
    let data: String = redis.get(&symbol).await.unwrap();
    let data: schema::DepthStreamData = serde_json::from_str(&data).unwrap();
    Json(data)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/book", routes![tip])
        .attach(RedisPool::init())
}