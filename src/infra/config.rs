use envconfig::Envconfig;

#[derive(Debug, Clone, Envconfig)]
pub struct AppConfig {
    #[envconfig(from = "APP_NAME")]
    pub app_name: String,

    #[envconfig(from = "APP_PORT")]
    pub app_port: u16,

    #[envconfig(from = "APP_ENV")]
    pub app_env: String,

    #[envconfig(from = "DATABASE_URL")]
    pub db_url: String,

    #[envconfig(from = "REDIS_URL")]
    pub redis_url: String,

    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: String,

    #[envconfig(from = "ALLOWED_ORIGINS")]
    pub allowed_origins: String,

    #[envconfig(from = "GOOGLE_CLIENT_ID")]
    pub google_client_id: String,

    #[envconfig(from = "GOOGLE_CLIENT_SECRET")]
    pub google_client_secret: String,

    #[envconfig(from = "GOOGLE_REDIRECT_URI")]
    pub google_redirect_url: String,

    #[envconfig(from = "SUPER_KEY")]
    pub super_key: String,
}
