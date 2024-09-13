use crate::{
    domain::entities::user_oauth_provider::UserOauthProvider, infra::errors::app_error::AppError,
};

#[async_trait::async_trait]
pub trait OauthProviderRepository {
    async fn get_by_provider_and_id(
        &self,
        provider: &str,
        provider_id: &str,
    ) -> Result<UserOauthProvider, AppError>;
}
