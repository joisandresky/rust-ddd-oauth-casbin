use std::sync::Arc;

use crate::domain::repositories::redis_repo::RedisRepository;

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
}
