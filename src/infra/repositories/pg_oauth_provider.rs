use crate::{
    domain::{
        entities::user_oauth_provider::UserOauthProvider,
        repositories::oauth_provider_repo::OauthProviderRepository,
    },
    infra::errors::app_error::AppError,
};

#[derive(Clone, Debug)]
pub struct PgOauthProviderRepository {
    db_pool: sqlx::PgPool,
}

impl PgOauthProviderRepository {
    pub fn new(db_pool: sqlx::PgPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait::async_trait]
impl OauthProviderRepository for PgOauthProviderRepository {
    async fn get_by_provider_and_id(
        &self,
        provider: &str,
        provider_id: &str,
    ) -> Result<UserOauthProvider, AppError> {
        let oauth_provider = sqlx::query_as!(
            UserOauthProvider,
            "SELECT * FROM user_oauth_providers WHERE provider = $1 AND provider_user_id = $2",
            provider,
            provider_id
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(oauth_provider)
    }
}
