use crate::handlers::auth_handler;
use axum::{Router, routing::post};
use sqlx::PgPool;
pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
        .route("/refreh-token", post(auth_handler::refresh_token))
}
