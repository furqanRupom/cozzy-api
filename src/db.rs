use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

pub async fn connect_db() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
    PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect with DB")
}
