// src/models/response.rs
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T = ()> {
    pub status: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            status: true,
            message: message.into(),
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            status: false,
            message: message.into(),
            data: None,
        }
    }
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
}
