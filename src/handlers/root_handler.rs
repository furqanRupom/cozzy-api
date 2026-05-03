use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: u16,
    pub success: bool,
    pub message: String,
}

pub async fn root() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: 200,
            success: true,
            message: "Cozzy API is running".to_string(),
        }),
    )
}
