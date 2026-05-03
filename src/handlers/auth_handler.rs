use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::models::user::{AuthResponse, LoginUser, RegisterUser};
use crate::services::auth_service;

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterUser>,
) -> Json<AuthResponse> {
    let _ = auth_service::register_user(pool, payload).await;
    Json(AuthResponse {
        message: "User Registerd successfully".to_string(),
        token: None,
    })
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginUser>,
) -> Json<AuthResponse> {
    let _ = auth_service::login_user(pool, payload).await;
    Json(AuthResponse {
        message: "User Logged In Successfully".to_string(),
        token: None,
    })
}
