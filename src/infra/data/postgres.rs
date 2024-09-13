use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub async fn establish_connection(db_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(8)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .connect(db_url)
        .await
        .expect("failed to connect to database")
}
