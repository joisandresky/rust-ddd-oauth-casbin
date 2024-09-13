use std::sync::Arc;

use envconfig::Envconfig;
use rust_ddd_oauth_casbin::infra::{config::AppConfig, server::ServerBuilder};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // load env variables
    dotenvy::dotenv().ok();

    // populate env value into our config
    let cfg = AppConfig::init_from_env().expect("failed to load config from env");

    // init tracing for logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_ddd_oauth_casbin=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let server = ServerBuilder::new(Arc::new(cfg));

    server.run().await;
}
