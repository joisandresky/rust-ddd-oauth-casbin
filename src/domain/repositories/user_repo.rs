use crate::{
    domain::entities::{user::User, user_oauth_provider::UserOauthProvider, user_role::UserRole},
    infra::errors::app_error::AppError,
};

#[async_trait::async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: &str) -> Result<User, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<User, AppError>;
    async fn tx_create(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        entity: User,
    ) -> Result<User, AppError>;
    async fn tx_register_user(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        user: &User,
        user_oauth_provider: &UserOauthProvider,
        user_role: &UserRole,
    ) -> Result<(User, UserOauthProvider, UserRole), AppError>;
}
