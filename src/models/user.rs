use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
#[allow(dead_code)]
pub struct AuthResponse {
    pub message: String,
    pub token: Option<String>,
}
#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct UserLoginResponse {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RefreshToken {
    pub token: String,
}
