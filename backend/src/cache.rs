use std::env;

use redis::AsyncCommands;

pub async fn init() -> redis::Client {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL should be set");
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_multiplexed_tokio_connection().await.unwrap();
    let _: () = con.ping().await.unwrap();
    client
}
