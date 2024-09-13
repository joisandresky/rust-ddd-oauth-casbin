#[derive(serde::Deserialize, Debug, Clone)]
pub struct Oauth2Request {
    pub code: String,
}
