use std::{collections::HashMap, sync::Arc};

use crate::{
    domain::repositories::redis_repo::RedisRepository, infra::errors::app_error::AppError,
};

#[derive(Clone)]
pub struct RedisService<R> {
    redis_repo: Arc<R>,
}

impl<R> RedisService<R>
where
    R: RedisRepository,
{
    pub fn new(redis_repo: Arc<R>) -> Self {
        Self { redis_repo }
    }

    pub async fn get_permissions_list(&self) -> Result<HashMap<String, Vec<String>>, AppError> {
        let permissions_str = self.redis_repo.get_value("permissions").await?;
        let permissions: HashMap<String, Vec<String>> = serde_json::from_str(&permissions_str)?;

        Ok(permissions)
    }

    pub async fn set_permissions_list(
        &self,
        permissions: HashMap<String, Vec<String>>,
    ) -> Result<(), AppError> {
        let permissions_json = serde_json::to_string(&permissions)?;
        self.redis_repo
            .set_value_with_expiry("permissions", &permissions_json, 3600)
            .await?;

        Ok(())
    }
}
