use crate::infra::errors::app_error::AppError;
use crate::infra::oauth2::google::GOOGLE_OAUTH_ENDPOINT;

#[derive(Clone, Debug)]
pub struct GetGoogleAuthUrl {
    client_id: String,
    redirect_uri: String,
}

impl GetGoogleAuthUrl {
    pub fn new(client_id: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            redirect_uri,
        }
    }

    pub async fn execute(&self) -> Result<String, AppError> {
        let scopes = [
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ];

        let url = format!(
            "{}?client_id={}&redirect_uri={}&scope={}&prompt=consent&response_type=code&access_type=offline",
            GOOGLE_OAUTH_ENDPOINT, self.client_id, self.redirect_uri,
            scopes.join(" ")
        );

        Ok(url)
    }
}
