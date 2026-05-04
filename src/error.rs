// src/error.rs
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub struct AppError(pub StatusCode, pub String);

impl AppError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        AppError(status, message.into())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        eprintln!("Database error: {:?}", err);

        if let sqlx::Error::Database(db_err) = &err {
            if db_err.code().as_deref() == Some("23505") {
                return AppError(StatusCode::CONFLICT, "Email already exists".to_string());
            }
        }
        AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let AppError(status, message) = self;

        let body = Json(json!({
            "success": false,
            "error": message
        }));

        (status, body).into_response()
    }
}
