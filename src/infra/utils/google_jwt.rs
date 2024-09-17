use std::sync::{Arc, RwLock};

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::Deserialize;
use tokio::time::{Duration, Instant};

use crate::{
    application::dto::auth::jwt_claims::GoogleJwtClaims,
    infra::{config::AppConfig, errors::app_error::AppError},
};

#[derive(Clone)]
pub struct GoogleJwtMaker {
    cfg: Arc<AppConfig>,
    jwks_cache: Arc<RwLock<Option<JwksCache>>>,
}

#[derive(Deserialize, Clone)]
struct Jwk {
    // alg: String,
    kid: String,
    n: String,
    e: String,
    // kty: String,
}

#[derive(Deserialize, Clone)]
struct Jwks {
    keys: Vec<Jwk>,
}

struct JwksCache {
    jwks: Jwks,
    expires_at: Instant,
}

impl GoogleJwtMaker {
    pub fn new(cfg: Arc<AppConfig>) -> Self {
        Self {
            cfg,
            jwks_cache: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn verify_token(&self, token: &str) -> Result<GoogleJwtClaims, AppError> {
        let header = decode_header(token)?;
        let kid = match header.kid {
            Some(kid) => kid,
            None => return Err(AppError::InvalidToken),
        };

        let jwks = self.get_jwks().await?;

        let jwk = self
            .find_jwk_by_kid(&jwks, &kid)
            .ok_or(AppError::InvalidToken)?;

        let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.cfg.google_client_id]);
        validation.set_issuer(&["https://accounts.google.com"]);

        let decoded = decode::<GoogleJwtClaims>(token, &decoding_key, &validation)?;

        Ok(decoded.claims)
    }

    async fn get_jwks(&self) -> Result<Jwks, AppError> {
        {
            let cache = self
                .jwks_cache
                .read()
                .map_err(|err| AppError::ProcessError(err.to_string()))?;

            if let Some(cached) = &*cache {
                if cached.expires_at > Instant::now() {
                    return Ok(cached.jwks.clone());
                }
            }
        }

        let fresh_jwks = self.fetch_google_jwks().await?;

        {
            let mut cache = self
                .jwks_cache
                .write()
                .map_err(|err| AppError::ProcessError(err.to_string()))?; // Acquire write lock
            *cache = Some(JwksCache {
                jwks: fresh_jwks.clone(),
                expires_at: Instant::now() + Duration::from_secs(3600), // Cache for 1 hour
            });
        }

        Ok(fresh_jwks)
    }

    async fn fetch_google_jwks(&self) -> Result<Jwks, AppError> {
        let client = Client::new();
        let jwks_url = "https://www.googleapis.com/oauth2/v3/certs";
        let res = client.get(jwks_url).send().await?.json::<Jwks>().await?;

        Ok(res)
    }

    fn find_jwk_by_kid<'a>(&self, jwks: &'a Jwks, kid: &str) -> Option<&'a Jwk> {
        jwks.keys.iter().find(|key| key.kid == kid)
    }
}
