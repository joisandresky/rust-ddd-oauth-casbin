use std::{collections::HashMap, sync::Arc};

use crate::{
    domain::{entities::user::UserFull, repositories::redis_repo::RedisRepository},
    infra::errors::app_error::AppError,
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

    pub async fn set_current_user(&self, user: &UserFull) -> Result<(), AppError> {
        let redis_key = format!("current_user_{}", user.user.id);
        let user_json = serde_json::to_string(&user)?;
        self.redis_repo
            .set_value_with_expiry(&redis_key, &user_json, 3600)
            .await?;

        Ok(())
    }

    pub async fn get_current_user(&self, user_id: &str) -> Result<UserFull, AppError> {
        let redis_key = format!("current_user_{}", user_id);
        let user_str = self.redis_repo.get_value(&redis_key).await?;
        let user: UserFull = serde_json::from_str(&user_str)?;

        Ok(user)
    }

    pub async fn remove_current_user(&self, user_id: &str) -> Result<(), AppError> {
        let redis_key = format!("current_user_{}", user_id);
        self.redis_repo.delete_value(&redis_key).await?;

        Ok(())
    }
}
