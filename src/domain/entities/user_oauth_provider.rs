use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserOauthProvider {
    pub id: String,
    pub user_id: String,
    pub provider: String,
    pub provider_user_id: String,
}

impl UserOauthProvider {
    pub fn new(user_id: String, provider: String, provider_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            provider,
            provider_user_id: provider_id,
        }
    }

    pub fn update(&mut self, provider: String, provider_id: String) {
        self.provider = provider;
        self.provider_user_id = provider_id;
    }
}
