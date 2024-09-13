use crate::{
    domain::{
        entities::user::User, entities::user_oauth_provider::UserOauthProvider,
        entities::user_role::UserRole, repositories::user_repo::UserRepository,
    },
    infra::errors::app_error::AppError,
};

#[derive(Clone, Debug)]
pub struct PgUserRepository {
    pool: sqlx::PgPool,
}

impl PgUserRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<User, AppError> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: &str) -> Result<User, AppError> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn tx_create(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        entity: crate::domain::entities::user::User,
    ) -> Result<crate::domain::entities::user::User, AppError> {
        let user = sqlx::query_as!(
            crate::domain::entities::user::User,
            "INSERT INTO users (id, email, password_hash, fullname, avatar_url, is_active, created_at, updated_at, deleted_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
            entity.id,
            entity.email,
            entity.password_hash,
            entity.fullname,
            entity.avatar_url,
            entity.is_active,
            entity.created_at,
            entity.updated_at,
            entity.deleted_at
        ).fetch_one(&mut **tx).await?;

        Ok(user)
    }

    async fn tx_register_user(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        user: &User,
        user_oauth_provider: &UserOauthProvider,
        user_role: &UserRole,
    ) -> Result<(User, UserOauthProvider, UserRole), AppError> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (id, email, password_hash, fullname, avatar_url, is_active, created_at, updated_at, deleted_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
            user.id,
            user.email,
            user.password_hash,
            user.fullname,
            user.avatar_url,
            user.is_active,
            user.created_at,
            user.updated_at,
            user.deleted_at
        ).fetch_one(&mut **tx).await?;

        let user_oauth_provider = sqlx::query_as!(
            UserOauthProvider,
            "INSERT INTO user_oauth_providers (id, user_id, provider, provider_user_id) VALUES ($1, $2, $3, $4) RETURNING *",
            user_oauth_provider.id,
            user_oauth_provider.user_id,
            user_oauth_provider.provider,
            user_oauth_provider.provider_user_id
        ).fetch_one(&mut **tx).await?;

        let user_role = sqlx::query_as!(
            UserRole,
            "INSERT INTO user_roles (user_id, role_id, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING *",
            user_role.user_id,
            user_role.role_id,
            user_role.created_at,
            user_role.updated_at,
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok((user, user_oauth_provider, user_role))
    }
}
