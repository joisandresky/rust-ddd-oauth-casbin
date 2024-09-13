use crate::{domain::entities::user_session::UserSession, infra::errors::app_error::AppError};

#[async_trait::async_trait]
pub trait UserSessionRepository {
    async fn find_by_user_id(&self, user_id: &str) -> Result<UserSession, AppError>;
    async fn create(&self, entity: UserSession) -> Result<UserSession, AppError>;
    async fn update_refresh_token(&self, session: &UserSession) -> Result<(), AppError>;
}
