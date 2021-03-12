use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn connect(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().max_connections(5).connect(url).await
}
