use redis::{Client,RedisResult};

pub fn redis_client() -> RedisResult<Client> {
    Ok(redis::Client::open("redis://127.0.0.1/")?)
}