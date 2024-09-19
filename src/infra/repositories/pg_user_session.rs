use crate::{
    domain::{
        entities::user_session::UserSession, repositories::user_session_repo::UserSessionRepository,
    },
    infra::errors::app_error::AppError,
};

#[derive(Clone, Debug)]
pub struct PgUserSessionRepository {
    pool: sqlx::PgPool,
}

impl PgUserSessionRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserSessionRepository for PgUserSessionRepository {
    async fn find_by_user_id(&self, user_id: &str) -> Result<UserSession, AppError> {
        let sessions = sqlx::query_as!(
            UserSession,
            "SELECT * FROM user_sessions WHERE user_id = $1",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(sessions)
    }

    async fn find_by_refresh_token(&self, refresh_token: &str) -> Result<UserSession, AppError> {
        let sessions = sqlx::query_as!(
            UserSession,
            "SELECT * FROM user_sessions WHERE refresh_token = $1",
            refresh_token
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(sessions)
    }

    async fn create(&self, entity: UserSession) -> Result<UserSession, AppError> {
        let session = sqlx::query_as!(
            UserSession,
            "INSERT INTO user_sessions (id, user_id, access_token, refresh_token, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            entity.id,
            entity.user_id,
            entity.access_token,
            entity.refresh_token,
            entity.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    async fn update_token(&self, session: &UserSession) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE user_sessions SET refresh_token = $1, access_token = $2 WHERE user_id = $3",
            session.refresh_token,
            session.access_token,
            session.user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
