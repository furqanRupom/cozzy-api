use crate::models::user::{AuthResponse, LoginUser, RegisterUser};
use crate::services::auth_service;
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterUser>,
) -> (StatusCode, Json<AuthResponse>) {
    match auth_service::register_user(pool, payload).await {
        Ok(message) => (
            StatusCode::CREATED,
            Json(AuthResponse {
                message,
                token: None,
            }),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(AuthResponse {
                message: e.to_string(),
                token: None,
            }),
        ),
    }
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginUser>,
) -> (StatusCode, Json<AuthResponse>) {
    match auth_service::login_user(pool, payload).await {
        Ok(result) => {
            if result.contains("Invalid") {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(AuthResponse {
                        message: result,
                        token: None,
                    }),
                )
            } else {
                // Assume it's a token
                (
                    StatusCode::OK,
                    Json(AuthResponse {
                        message: "Login successful".to_string(),
                        token: Some(result),
                    }),
                )
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                message: e.to_string(),
                token: None,
            }),
        ),
    }
}
