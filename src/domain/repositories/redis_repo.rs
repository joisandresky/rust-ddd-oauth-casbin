use crate::infra::errors::app_error::AppError;

#[async_trait::async_trait]
pub trait RedisRepository {
    async fn get_value(&self, key: &str) -> Result<String, AppError>;
    async fn set_value(&self, key: &str, value: &str) -> Result<(), AppError>;
    async fn set_value_with_expiry(
        &self,
        key: &str,
        value: &str,
        expiry: u64,
    ) -> Result<(), AppError>;
    async fn delete_value(&self, key: &str) -> Result<(), AppError>;
    async fn set_expiry(&self, key: &str, expiry: i64) -> Result<(), AppError>;
}
