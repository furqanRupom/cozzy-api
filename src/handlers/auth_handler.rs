use crate::error::AppError;
use crate::models::user::{LoginUser, RefreshToken, RegisterUser};
use crate::services::auth_service;
use crate::shared::response::{ApiResponse, TokenResponse};
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterUser>,
) -> Result<(StatusCode, Json<ApiResponse>), AppError> {
    let message = auth_service::register_user(pool, payload).await?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(message, ()))))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginUser>,
) -> Result<(StatusCode, Json<ApiResponse<TokenResponse>>), AppError> {
    let token = auth_service::login_user(pool, payload).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "User Logged in successfully",
            TokenResponse {
                access_token: token,
            },
        )),
    ))
}
pub async fn refresh_token(
    State(pool): State<PgPool>,
    Json(payload): Json<RefreshToken>,
) -> Result<(StatusCode, Json<ApiResponse<TokenResponse>>), AppError> {
    let token = auth_service::refresh_token_user(pool, payload).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            "Token refreshed successfully",
            TokenResponse {
                access_token: token,
            },
        )),
    ))
}
