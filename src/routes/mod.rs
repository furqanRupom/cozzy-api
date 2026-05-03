pub mod auth;
use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::root_handler;

pub fn create_routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(root_handler::root))
        .nest("/api/auth", auth::routes())
}
