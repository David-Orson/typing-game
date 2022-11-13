use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub mod account;
pub mod auth;
mod migration;

pub async fn open() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    migration::up(&pool).await;

    pool
}
