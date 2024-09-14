use bb8_redis::{bb8::Pool, redis::AsyncCommands, RedisConnectionManager};

use crate::{
    domain::repositories::redis_repo::RedisRepository, infra::errors::app_error::AppError,
};

#[derive(Clone, Debug)]
pub struct RedisRepositoryImpl {
    pool: Pool<RedisConnectionManager>,
}

impl RedisRepositoryImpl {
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RedisRepository for RedisRepositoryImpl {
    async fn get_value(&self, key: &str) -> Result<String, AppError> {
        let mut conn = self.pool.get().await?;

        let value = conn.get(key).await?;

        Ok(value)
    }

    async fn set_value(&self, key: &str, value: &str) -> Result<(), AppError> {
        let mut conn = self.pool.get().await.unwrap();

        let _ = conn.set(key, value).await?;

        Ok(())
    }

    async fn set_value_with_expiry(
        &self,
        key: &str,
        value: &str,
        expiry: u64,
    ) -> Result<(), AppError> {
        let mut conn = self.pool.get().await.unwrap();

        let _ = conn.set_ex(key, value, expiry).await?;

        Ok(())
    }

    async fn delete_value(&self, key: &str) -> Result<(), AppError> {
        let mut conn = self.pool.get().await.unwrap();

        let _ = conn.del(key).await?;

        Ok(())
    }

    async fn set_expiry(&self, key: &str, expiry: i64) -> Result<(), AppError> {
        let mut conn = self.pool.get().await.unwrap();

        let _ = conn.expire(key, expiry).await?;

        Ok(())
    }
}
