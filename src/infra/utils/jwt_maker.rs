use tracing::error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
    pub iss: String,
    pub name: String,
    pub roles: Vec<String>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RefreshTokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
    pub iss: String,
}

#[derive(Clone, Debug)]
pub struct JwtMaker {
    secret: String,
}

impl JwtMaker {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn make_token(
        &self,
        user_id: String,
        expiration_hours: i64,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        // create claims with expiration time for 7 days
        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::hours(expiration_hours);
        let claims = Claims {
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            sub: user_id.clone(),
            iss: "API_NAME".to_owned(),
            name: String::default(),
            roles: vec![],
            scopes: vec![],
        };

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn make_refresh_token(
        &self,
        user_id: String,
        expiration_hours: i64,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        // create claims with expiration time for 7 days
        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::hours(expiration_hours);
        let claims = RefreshTokenClaims {
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            sub: user_id.clone(),
            iss: "API_NAME".to_owned(),
        };

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn verify_access_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|err| {
            error!("[JWT->verify_token] Failed to verify token: {}", err);
            err
        })?;

        Ok(claims.claims)
    }

    pub fn verify_refresh_token(
        &self,
        token: &str,
    ) -> Result<RefreshTokenClaims, jsonwebtoken::errors::Error> {
        let claims = jsonwebtoken::decode::<RefreshTokenClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|err| {
            error!("[JWT->verify_token] Failed to verify token: {}", err);
            err
        })?;

        Ok(claims.claims)
    }
}
