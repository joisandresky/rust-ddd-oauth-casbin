use bb8_redis::{
    bb8::{self, Pool},
    RedisConnectionManager,
};

pub async fn get_redis_con(redis_url: &str) -> Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new(redis_url).unwrap();

    bb8::Pool::builder().build(manager).await.unwrap()
}
